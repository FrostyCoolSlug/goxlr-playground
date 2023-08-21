use anyhow::{bail, Context, Result};
use log::{debug, error, warn};
use tokio::sync::{mpsc, oneshot};
use tokio::{join, select, task};

use goxlr_profile::Profile;
use goxlr_shared::colours::ColourScheme;
use goxlr_shared::device::DeviceInfo;
use goxlr_shared::interaction::ButtonStates;
use goxlr_shared::states::ButtonDisplayStates;
use goxlr_usb_messaging::events::commands::{BasicResultCommand, CommandSender};
use goxlr_usb_messaging::runners::device::DeviceMessage;
use goxlr_usb_messaging::runners::device::{start_usb_device_runner, GoXLRUSBConfiguration};

use crate::device::device_manager::{RunnerMessage, RunnerState};
use crate::device::goxlr::device_config::GoXLRDeviceConfiguration;
use crate::device::goxlr::parts::load_profile::LoadProfile;
use crate::stop::Stop;

pub(crate) struct GoXLR {
    device: Option<DeviceInfo>,
    command_sender: Option<mpsc::Sender<CommandSender>>,

    pub profile: Profile,
    pub colour_scheme: ColourScheme,
    pub button_states: ButtonDisplayStates,

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

        // Prepare all the messaging queues that are needed..
        let (event_send, mut event_recv) = mpsc::channel(16);
        let (command_send, command_recv) = mpsc::channel(32);
        let (interaction_send, mut interaction_recv) = mpsc::channel(128);
        let (stop_send, stop_recv) = oneshot::channel();
        let (ready_send, ready_recv) = oneshot::channel();

        let configuration = GoXLRUSBConfiguration {
            device: self.config.device,
            interaction_event: interaction_send,
            device_event: event_send,
            command_receiver: command_recv,
            stop: stop_recv,
        };
        let runner = task::spawn(start_usb_device_runner(configuration, ready_send));

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

        self.load_profile().await?;

        loop {
            select! {
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
                    debug!("Something Chnaged! {:?}", event);
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

        // Our loop has been broken, let the device know we're done..
        let _ = stop_send.send(());
        debug!("[GoXLR]{} Event Loop Ended", self.config.device);

        debug!(
            "[GoXLR]{} Waiting for Device Runner to stop..",
            self.config.device
        );
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
