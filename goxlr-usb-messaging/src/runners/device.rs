/*
   This object essentially acts as a 'proxy' struct between external code, and the actual device
   using messaging to get things backwards and forwards
*/

use crate::platform::rusb::device::{GoXLRConfiguration, GoXLRDevice};
use crate::USBLocation;
use anyhow::Result;
use log::{debug, error};
use tokio::select;
use tokio::sync::{mpsc, oneshot};

// This is an obnoxiously long type, shorten it!
type Ready = oneshot::Sender<mpsc::Sender<bool>>;

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

                    // We've received an event from the device, simply propagate it straight up.
                    let _ = self.config.events.send(event).await;
                }
                Some(command) = msg_recv.recv() => {
                    debug!("[RUNNER]{} Received Command: {:?}", command, self.config.device);
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
    Event,
}
