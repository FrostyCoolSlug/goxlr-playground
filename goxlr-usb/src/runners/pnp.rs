/*
   This is the PnP runner for the GoXLR, it simply detects and tracks devices which have been
   attached and removed, and sends messages accordingly. It's the responsibility of the event
   device runner to handle errors gracefully.
*/

use std::time::Duration;

use log::{info, warn};
use tokio::sync::{mpsc, oneshot};
use tokio::{select, time};

use crate::platform::libusb::pnp::get_devices;
use crate::USBLocation;

struct PnPRunner {
    config: PnPConfiguration,
    device_list: Vec<USBLocation>,
}

impl PnPRunner {
    fn new(config: PnPConfiguration) -> Self {
        Self {
            config,
            device_list: vec![],
        }
    }

    pub async fn run(&mut self) {
        // Create a timer for re-checking for devices..
        let mut ticker = time::interval(Duration::from_millis(500));
        info!("[PNP] Started Monitoring for Devices");
        loop {
            select! {
                msg = &mut self.config.stop_signal => {
                    match msg {
                        Ok(_) => {
                            info!("[PNP] Stop Requested");
                        }
                        Err(e) => {
                            warn!("[PNP] Stop signal lost! Aborting: {}", e);
                        }
                    }
                    break;
                },

                _ = ticker.tick() => {
                    let devices = get_devices().await;
                    self.handle_devices(devices).await;
                }
            }
        }
        info!("[PNP] Handler Ended");
    }

    async fn handle_devices(&mut self, devices: Vec<USBLocation>) {
        let sender = &self.config.device_sender;
        // Look for any devices that aren't present in our list..
        for device in &devices {
            if !self.device_list.contains(device) {
                // New device found, send a message to the device sender..
                info!("[PnP] New GoXLR Device found: {:?}", device);
                let _ = sender.send(PnPDeviceMessage::Attached(*device)).await;
                self.device_list.push(*device);
            }
        }

        // Check for device removals, we could iter and remove as we went, but you can't trigger
        // an async inside a closure, so we'll flag it here, and clean it after
        let mut device_removed = false;
        for device in &self.device_list {
            if !devices.contains(device) {
                // Device has been removed, send out the event.
                info!("[PnP] GoXLR Device has been removed: {:?}", device);
                let _ = sender.send(PnPDeviceMessage::Removed(*device)).await;
                device_removed = true;
            }
        }

        // A device has been removed, simply retain all those that haven't
        if device_removed {
            self.device_list.retain(|device| {
                if !devices.contains(device) {
                    return false;
                }
                true
            });
        }
    }
}

pub async fn start_pnp_runner(config: PnPConfiguration) {
    let mut runner = PnPRunner::new(config);
    runner.run().await;
}

pub struct PnPConfiguration {
    pub stop_signal: oneshot::Receiver<()>,
    pub device_sender: mpsc::Sender<PnPDeviceMessage>,
}

pub enum PnPDeviceMessage {
    Attached(USBLocation),
    Removed(USBLocation),
}
