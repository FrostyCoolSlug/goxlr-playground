use std::time::Duration;

use anyhow::{bail, Context, Result};
use enum_map::EnumMap;
use goxlr_ipc::commands::GoXLRCommandResponse;
use log::{debug, error, warn};
use tokio::sync::{mpsc, oneshot};
use tokio::{join, select, task, time};

use goxlr_profile::Profile;
use goxlr_shared::buttons::Buttons;
use goxlr_shared::channels::ChannelMuteState;
use goxlr_shared::colours::ColourScheme;
use goxlr_shared::device::DeviceInfo;
use goxlr_shared::faders::{Fader, FaderSources};
use goxlr_shared::routing::RoutingTable;
use goxlr_shared::states::ButtonDisplayStates;
use goxlr_usb::events::commands::{BasicResultCommand, CommandSender};
use goxlr_usb::events::interaction::InteractionEvent;
use goxlr_usb::runners::device::DeviceMessage;
use goxlr_usb::runners::device::{start_usb_device_runner, GoXLRUSBConfiguration};

use crate::device::device_manager::{ManagerMessage, RunnerMessage, RunnerState};
use crate::device::goxlr::components::interactions::Interactions;
use crate::device::goxlr::components::load_profile::LoadProfile;
use crate::device::goxlr::device_config::GoXLRDeviceConfiguration;
use crate::device::goxlr::ipc::handler::IPCCommandHandler;
use crate::stop::Stop;

pub(crate) struct GoXLR {
    pub device: Option<DeviceInfo>,
    command_sender: Option<mpsc::Sender<CommandSender>>,

    pub profile: Profile,

    // These are 'caches' of the state which are manipulated directly.
    pub colour_scheme: ColourScheme,
    pub button_states: ButtonDisplayStates,
    pub routing_state: RoutingTable,
    pub mute_state: EnumMap<FaderSources, Option<ChannelMuteState>>,
    pub fader_state: EnumMap<Fader, Option<FaderSources>>,

    // For tracking button 'held' state..
    pub button_down_states: EnumMap<Buttons, Option<ButtonState>>,

    config: GoXLRDeviceConfiguration,
    shutdown: Stop,
}

impl GoXLR {
    pub fn new(config: GoXLRDeviceConfiguration, shutdown: Stop) -> Self {
        Self {
            device: None,
            command_sender: None,

            colour_scheme: Default::default(),
            profile: Default::default(),
            button_states: Default::default(),
            routing_state: Default::default(),
            mute_state: Default::default(),
            fader_state: Default::default(),
            button_down_states: Default::default(),

            config,
            shutdown,
        }
    }

    /// This function is simply for command which only have a simple Success / Failed result,
    /// there's ultimately no need to have loads of set up / tear down code for the messaging
    /// system all over the place if we're not expecting to handle anything.
    pub(crate) async fn send_no_result(&self, command: BasicResultCommand) -> Result<()> {
        let (msg_send, msg_receive) = oneshot::channel();

        let command_sender = self.command_sender.clone();
        let sender = command_sender.context("Sender not configured!")?;

        // Send the message..
        let command = CommandSender::BasicResultCommand(command, msg_send);
        let _ = sender.send(command).await;

        // Return the Response..
        msg_receive.await?
    }

    pub async fn run(&mut self) -> Result<()> {
        debug!("[GoXLR]{} Starting Event Loop", self.config.device);

        // These are device specific messages sent to us by the handler..
        let (event_send, mut event_recv) = mpsc::channel(16);

        // This is the command channel, for sending commands, and receiving responses from the device
        let (command_send, command_recv) = mpsc::channel(32);

        // These are callbacks for physical interactions with the device (Buttons Pressed / Volumes Changed)
        let (interaction_send, mut interaction_recv) = mpsc::channel(128);

        // A signalling channel to tell the device workers to stop
        let (stop_send, stop_recv) = oneshot::channel();

        // A signal from the device runner to tell us it's ready to go.
        let (ready_send, ready_recv) = oneshot::channel();

        // A ticker to handle internal data handling periodically.
        let mut ticker = time::interval(Duration::from_millis(20));

        // Build the configuration for the USB Runner, with the relevant messaging queues
        let configuration = GoXLRUSBConfiguration {
            device: self.config.device,
            interaction_event: Some(interaction_send),
            device_event: event_send,
            command_receiver: command_recv,
            stop: stop_recv,
        };
        let runner = task::spawn(start_usb_device_runner(configuration, ready_send));

        // Use the ready signal to hold here, until the usb running is running, this will also
        // provide us with the device info (such as serial, features, versions, etc).
        let device = match ready_recv.await {
            Ok(recv) => recv,
            Err(e) => {
                bail!("Error on Starting Receiver, aborting: {}", e);
            }
        };

        let serial = device.serial.clone();
        self.device = Some(device);
        self.command_sender = Some(command_send);

        // Let the device runner know we're up and running
        let run_msg = RunnerMessage::UpdateState(self.config.device, RunnerState::Running(serial));
        let _ = self.config.manager_sender.send(run_msg).await;

        // Load the profile.
        let mut load_fail = false;
        if let Err(error) = self.load_profile().await {
            warn!("Error While Loading Profile: {}", error);
            load_fail = true;
        }

        // Only enter the loop if we were able to load the profile, otherwise immediately abort and
        // shut down the runners. We shouldn't just bail if there's an error above, as the USB
        // runtime has already been started, the easiest way to stop it is to just jump to the end.
        if !load_fail {
            // Sit and wait for various signals to come, and process them as they do.
            loop {
                select! {
                    Some(event) = self.config.manager_recv.recv() => {
                        match event {
                            ManagerMessage::Execute(command, tx) => {
                                debug!("Handling IPC Command: {:?}", command);

                                let result = self.handle_ipc_command(command).await;
                                let message = match result {
                                    Ok(res) => res,
                                    Err(e) => {
                                        warn!("Execution Error: {}", e.to_string());
                                        GoXLRCommandResponse::Error(e.to_string())
                                    }
                                };
                                let _ = tx.send(message);
                            }
                        }
                    }
                    Some(event) = event_recv.recv() => {
                        debug!("[GoXLR]{} Event: {:?}", self.config.device, event);
                        match event {
                            DeviceMessage::Error => {
                                warn!("[GoXLR]{} Error Sent back from Handler, bail!", self.config.device);
                                break;
                            }
                            DeviceMessage::Event(event) => {
                                debug!("[GoXLR]{} Event: {:?}", self.config.device, event);
                            }
                        }
                    }
                    Some(event) = interaction_recv.recv() => {
                        let result = match event {
                            InteractionEvent::ButtonDown(button) => {
                                self.on_button_down(button.into()).await
                            },
                            InteractionEvent::ButtonUp(button) => {
                                self.on_button_up(button.into()).await
                            },
                            InteractionEvent::VolumeChange(fader, value) => {
                                self.on_volume_change(fader.into(), value).await
                            },
                            InteractionEvent::EncoderChange(encoder, value) => {
                                self.on_encoder_change(encoder.into(), value).await
                            }
                        };

                        if let Err(error) = result {
                            warn!("Error Handling Button Press: {:?}", error);
                        }
                    }
                    _ = ticker.tick() => {
                        // Things to do every 20ms..
                        let _ = self.check_held().await;

                        // // Lets grab the current db value of the Microphone..
                        // let (msg_send, msg_receive) = oneshot::channel();
                        //
                        // if let Some(sender) = self.command_sender.clone() {
                        //     let command = CommandSender::GetMicLevel(msg_send);
                        //     let _ = sender.send(command).await;
                        //
                        //     if let Ok(Ok(value)) = msg_receive.await {
                        //         debug!("{}", value);
                        //     }
                        // }
                    }
                    _ = self.shutdown.recv() => {
                        debug!("[GoXLR]{} Shutdown Triggered!", self.config.device);
                        break;
                    }
                    _ = self.config.stop.recv() => {
                        debug!("[GoXLR]{} Device Disconnected!", self.config.device);
                        break;
                    }
                }
            }
        }

        // Our loop has been broken (or never started), let the device know we're done..
        let device = self.config.device;

        let _ = stop_send.send(());
        debug!("[GoXLR]{} Event Loop Ended", device);

        debug!("[GoXLR]{} Waiting for Device Runner to stop..", device);
        let _ = join!(runner);

        debug!("[GoXLR]{} Runner Stopped", self.config.device);
        let run_msg = RunnerMessage::UpdateState(self.config.device, RunnerState::Stopped);
        let _ = self.config.manager_sender.send(run_msg).await;
        debug!("[GoXLR]{} Device Runtime Ended..", self.config.device);

        Ok(())
    }
}

pub async fn start_goxlr(config: GoXLRDeviceConfiguration, shutdown: Stop) {
    // Prepare an error handler, in case something goes wrong during init / runtime..
    let sender = config.manager_sender.clone();
    let error_msg = RunnerMessage::Error(config.device);

    let mut device = GoXLR::new(config, shutdown);
    if let Err(error) = device.run().await {
        error!("Error during device runtime: {}", error);
        let _ = sender.send(error_msg).await;
    }
}

/// This is a simple struct that tracks how long long a button has been pressed for..
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct ButtonState {
    pub(crate) press_time: u128,

    pub(crate) skip_hold: bool,
    pub(crate) skip_release: bool,
    pub(crate) hold_handled: bool,
}
