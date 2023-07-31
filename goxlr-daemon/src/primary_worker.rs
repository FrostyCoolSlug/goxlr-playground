use crate::device::device::{Device, DeviceInner};
use goxlr_usb::platform::unix::pnp::run_pnp_handler;
use goxlr_usb::pnp_base::DeviceEvents;
use goxlr_usb::GoXLRDevice;
use log::{error, info};
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;

// This is a somewhat cut down version of the primary worker, which offsets a lot of the checking
// (such as Plug and Play checking) to the USB crate, and Button Events into the device itself.
pub async fn run_worker() {
    let mut devices: HashMap<GoXLRDevice, Device> = HashMap::new();

    // Spawn the PnP handler..
    let (pnp_sender, mut pnp_receiver) = mpsc::channel(32);
    task::spawn(run_pnp_handler(pnp_sender.clone()));

    loop {
        tokio::select! {
            Some(event) = pnp_receiver.recv() => {
                match event {
                    DeviceEvents::Attached(goxlr_device) => {
                        info!("New Device Found: {:#?}", goxlr_device);

                        // Create the Device..
                        let device = Device::new();
                        match device {
                            Ok(device) => {
                                // Store this device as 'known'.
                                devices.insert(goxlr_device, device);
                            }
                            Err(e) => {
                                error!("Unable to Load GoXLR Device");
                            }
                        }

                    },
                    DeviceEvents::Removed(device) => {
                        info!("Device Removed: {:#?}", device);
                        devices.get_mut(&device).unwrap().stop().await;

                    },
                    DeviceEvents::Error(device) => {
                        // If it's not been detached, we should just destroy the existing device
                        // and re-create it, with hopes it'll cleanly reset.
                        info!("Device Errored: {:#?}", device);
                    },
                }
            }
        }
    }
}
