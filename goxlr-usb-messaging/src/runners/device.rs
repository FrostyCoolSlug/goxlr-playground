/*
   This object essentially acts as a 'proxy' struct between external code, and the actual device
   using messaging to get things backwards and forwards
*/

use anyhow::{bail, Result};
use log::{debug, error};
use tokio::select;
use tokio::sync::{mpsc, oneshot};

use crate::platform::rusb::device::{GoXLRConfiguration, GoXLRDevice};
use crate::requests::{GoXLRMessage, GoXLRStatus};
use crate::USBLocation;

// This is an obnoxiously long type, shorten it!
type Ready = oneshot::Sender<mpsc::Sender<GoXLRMessage>>;

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
        let (event_send, mut event_recv) = mpsc::channel(128);
        let (msg_send, mut msg_recv) = mpsc::channel(128);

        let config = GoXLRConfiguration {
            device: self.config.device,
            messenger: msg_send.clone(),
            events: event_send.clone(),
        };

        // Ok, firstly, we need to create a GoXLR device from our Location..
        debug!("[RUNNER]{} Initialising Device..", self.config.device);
        let mut device = GoXLRDevice::from(config).await?;
        device.initialise().await?;

        debug!(
            "[RUNNER]{} Device Initialised, starting event loop",
            self.config.device
        );

        // Once we get here, the device has setup, send back the message sender...
        let _ = ready.send(msg_send.clone());

        loop {
            select! {
                Some(event) = event_recv.recv() => {
                    debug!("[RUNNER]{} Event From Device!", self.config.device);

                    match event {
                        DeviceMessage::Error => {
                            bail!("Error in Message Handler, aborting");
                        },
                        DeviceMessage::Event(event) => {
                            match event {
                                EventType::Status(status) => {
                                    debug!("Received Status, do work: {:?}", status);
                                },
                            }
                        },
                    }
                }
                Some(command) = msg_recv.recv() => {
                    debug!("[RUNNER]{} Received Command: {:?}", self.config.device, command);
                    match command {
                        GoXLRMessage::GetStatus(response) => {
                            let _ = response.send(GoXLRStatus {});
                        },
                    }
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
}

pub async fn start_usb_device_runner(config: GoXLRUSBConfiguration, ready: Ready) {
    let sender = config.events.clone();

    let mut device = GoXLRUSBDevice::new(config);
    if device.run(ready).await.is_err() {
        let _ = sender.send(DeviceMessage::Error).await;
    }
}

pub struct GoXLRUSBConfiguration {
    pub device: USBLocation,
    pub events: mpsc::Sender<DeviceMessage>,
    pub stop: oneshot::Receiver<()>,
}

#[derive(Debug)]
pub enum DeviceMessage {
    Error,
    Event(EventType),
}

#[derive(Debug)]
pub enum EventType {
    Status(GoXLRStatus),
}
