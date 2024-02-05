/*
   This object essentially acts as a 'proxy' struct between external code, and the actual device
   using messaging to get things backwards and forwards
*/

use std::cmp::Ordering;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering as AtomicOrder;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{bail, Result};
use log::{debug, trace};
use strum::IntoEnumIterator;
use tokio::sync::{mpsc, oneshot};
use tokio::{select, time};

use goxlr_shared::device::{DeviceInfo, DeviceType, GoXLRFeature};
use goxlr_shared::interaction::{ButtonStates, CurrentStates};
use goxlr_shared::version::VersionNumber;

use crate::common::command_handler::GoXLRCommands;
use crate::events::commands::{BasicResultCommand, ChannelSource, CommandSender};
use crate::events::interaction::InteractionEvent;
use crate::handlers::state_tracker::StateTracker;
use crate::platform::rusb::device::{GoXLRConfiguration, GoXLRDevice};
use crate::types::channels::AssignableChannel;
use crate::types::encoders::DeviceEncoder;
use crate::types::faders::DeviceFader;
use crate::USBLocation;

// This is an obnoxiously long type, shorten it!
type Ready = oneshot::Sender<DeviceInfo>;

struct GoXLRUSBDevice {
    config: GoXLRUSBConfiguration,
}

impl GoXLRUSBDevice {
    pub fn new(config: GoXLRUSBConfiguration) -> Self {
        Self { config }
    }

    pub async fn run(&mut self, ready: Ready) -> Result<()> {
        debug!("[RUNNER]{} Starting Device Runner..", self.config.device);

        // Create an event receiver for the device..
        let (event_send, mut event_recv) = mpsc::channel(1);
        //let (msg_send, mut msg_recv) = mpsc::channel(128);

        // Create a state tracker to monitor for physical changes to the GoXLR..
        let mut tracker = None;
        if let Some(interaction) = &self.config.interaction_event {
            tracker = Some(StateTracker::new(interaction.clone()));
        }

        let config = GoXLRConfiguration {
            device: self.config.device,
            events: event_send.clone(),
        };

        // Ok, firstly, we need to create a GoXLR device from our Location..
        debug!("[RUNNER]{} Initialising Device..", self.config.device);
        let mut device = GoXLRDevice::from(config).await?;
        device.run().await?;

        debug!(
            "[RUNNER]{} Device Initialised, starting event loop",
            self.config.device
        );

        let details = self.get_device_info(&mut device).await?;

        // Once we get here, the device has setup, send back the message sender...
        let _ = ready.send(details);

        loop {
            select! {
                Some(event) = event_recv.recv() => {
                    trace!("[RUNNER]{} Event From Device: {:?}", self.config.device, event);

                    match event {
                        InternalDeviceMessage::Error => {
                            bail!("Error in Message Handler, aborting");
                        },
                        InternalDeviceMessage::Poll => {
                            // If the user hasn't defined a tracker, we do nothing. It becomes entirely
                            // their responsibility to manage it.
                            if let Some(tracker) = &mut tracker {
                                if !self.config.pause_interaction_poll.load(AtomicOrder::Relaxed) {
                                    let buttons = device.get_button_states().await?;
                                    tracker.update_states(buttons).await;
                                }
                            }
                        }
                    }
                }
                Some(command) = self.config.command_receiver.recv() => {
                    debug!("Handling Command..");
                    self.handle_command(command, &mut device).await;
                }
                _ = &mut self.config.stop => {
                    debug!("[RUNNER]{} Told to Stop, breaking Loop..", self.config.device);
                    break;
                }
            }
        }

        debug!("[RUNNER]{} Stopping device..", self.config.device);
        device.stop().await;

        debug!("[RUNNER]{} Event loop terminated.", self.config.device);
        Ok(())
    }

    async fn handle_command(&self, sender: CommandSender, device: &mut GoXLRDevice) {
        trace!("Running: {:#?}", sender);
        match sender {
            CommandSender::BasicResultCommand(command, responder) => match command {
                BasicResultCommand::SetColour(scheme) => {
                    let _ = responder.send(device.apply_colour_scheme(scheme).await);
                }
                BasicResultCommand::SetVolume(source, volume) => {
                    let channel = self.source_to_channel(source);
                    let _ = responder.send(device.set_volume(channel, volume).await);
                }
                BasicResultCommand::SetMuteState(source, state) => {
                    let channel = self.source_to_channel(source);
                    let _ = responder.send(device.set_mute_state(channel, state.into()).await);
                }
                BasicResultCommand::AssignFader(fader, source) => {
                    let channel = self.source_to_channel(source);
                    let _ = responder.send(device.assign_fader(fader.into(), channel).await);
                }
                BasicResultCommand::ApplyRouting(input, table) => {
                    let _ = responder.send(device.apply_routing(input, table).await);
                }
                BasicResultCommand::SetFaderStyle(fader, style) => {
                    let _ = responder.send(device.set_fader_style(fader, style).await);
                }
                BasicResultCommand::SetButtonStates(states) => {
                    let _ = responder.send(device.set_button_states(states).await);
                }
                BasicResultCommand::SetScribble(fader, data) => {
                    let _ = responder.send(device.set_scribble(fader, data).await);
                }
                BasicResultCommand::SetMicGain(mic_type, gain) => {
                    let _ = responder.send(device.set_microphone_gain(mic_type, gain).await);
                }
            },
            CommandSender::GetMicLevel(responder) => {
                let _ = responder.send(device.get_microphone_level().await);
            }

            CommandSender::GetButtonStates(responder) => {
                // Grab the states from the GoXLR..
                let states = device.get_button_states().await;
                match states {
                    Ok(states) => {
                        // Convert them to something that can be used upstream
                        let mut state = CurrentStates::default();
                        for button in states.pressed {
                            state.buttons[button.into()] = ButtonStates::Pressed;
                        }
                        for fader in DeviceFader::iter() {
                            state.volumes[fader.into()] = states.volumes[fader as usize];
                        }
                        for encoder in DeviceEncoder::iter() {
                            state.encoders[encoder.into()] = states.encoders[encoder as usize];
                        }
                    }
                    Err(error) => {
                        // Send the error upstream...
                        let _ = responder.send(Err(error));
                    }
                }
            }
        }
    }

    fn source_to_channel(&self, source: ChannelSource) -> AssignableChannel {
        match source {
            ChannelSource::FromInputChannel(source) => source.into(),
            ChannelSource::FromOutputChannel(source) => source.into(),
            ChannelSource::FromFaderSource(source) => source.into(),
            ChannelSource::FromVolumeChannel(source) => source.into(),
        }
    }

    pub async fn get_device_info(&self, device: &mut GoXLRDevice) -> Result<DeviceInfo> {
        // Ok, lets start pulling data..
        let (serial, manufacture_date) = device.get_serial_data().await?;
        let device_type = device.get_device_type();
        let firmware = device.get_firmware_version().await?;
        let mut features = vec![];

        let version = firmware.firmware;
        let (vod, animation, submix) = if device_type == DeviceType::Mini {
            // Mini Firmware Versions for Features..
            let vod = VersionNumber(1, 1, 10, 45);
            let animation = VersionNumber(1, 1, 8, 0);
            let submix = VersionNumber(1, 2, 0, 46);

            (vod, animation, submix)
        } else {
            // Full firmware versions for features..
            let vod = VersionNumber(1, 3, 43, 104);
            let animation = VersionNumber(1, 3, 40, 0);
            let submix = VersionNumber(1, 4, 2, 107);

            (vod, animation, submix)
        };

        if version == vod {
            features.push(GoXLRFeature::VoD);
        }
        if version_newer_or_equal_to(&version, animation) {
            features.push(GoXLRFeature::Animation);
        }
        if version_newer_or_equal_to(&version, submix) {
            features.push(GoXLRFeature::Submix);
        }

        Ok(DeviceInfo {
            serial,
            manufacture_date,
            device_type,
            firmware,
            features,
        })
    }
}

// Add all the executable Command handlers..
impl GoXLRCommands for GoXLRDevice {}

pub async fn start_usb_device_runner(config: GoXLRUSBConfiguration, ready: Ready) {
    let sender = config.device_event.clone();

    let mut device = GoXLRUSBDevice::new(config);
    if device.run(ready).await.is_err() {
        let _ = sender.send(DeviceMessage::Error).await;
    }
}

pub struct GoXLRUSBConfiguration {
    pub device: USBLocation,
    pub interaction_event: Option<mpsc::Sender<InteractionEvent>>,
    pub pause_interaction_poll: Arc<AtomicBool>,
    pub device_event: mpsc::Sender<DeviceMessage>,
    pub command_receiver: mpsc::Receiver<CommandSender>,
    pub stop: oneshot::Receiver<()>,
}

#[derive(Debug, Copy, Clone)]
pub enum DeviceMessage {
    Error,
}
#[derive(Debug)]
pub(crate) enum InternalDeviceMessage {
    Error,
    Poll,
}

/// This allows you to compare firmware version numbers against another specified version and
/// will return whether it's newer or equal to.
fn version_newer_or_equal_to(version: &VersionNumber, comparison: VersionNumber) -> bool {
    match version.0.cmp(&comparison.0) {
        Ordering::Greater => return true,
        Ordering::Less => return false,
        Ordering::Equal => {}
    }

    match version.1.cmp(&comparison.1) {
        Ordering::Greater => return true,
        Ordering::Less => return false,
        Ordering::Equal => {}
    }

    match version.2.cmp(&comparison.2) {
        Ordering::Greater => return true,
        Ordering::Less => return false,
        Ordering::Equal => {}
    }

    if version.3 >= comparison.3 {
        return true;
    }

    false
}
