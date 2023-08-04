use crate::device::device::{Device, DeviceInner};
use goxlr_usb::platform::unix::pnp::run_pnp_handler;
use goxlr_usb::pnp_base::DeviceEvents;
use goxlr_usb::GoXLRDevice;
use log::{error, info};
use std::cell::Cell;
use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::{task, time};

// This is a somewhat cut down version of the primary worker, which offsets a lot of the checking
// (such as Plug and Play checking) to the USB crate, and Button Events into the device itself.
pub async fn run_worker() {
    let mut device_list: Vec<GoXLRDevice> = Vec::new();
    let mut devices: HashMap<GoXLRDevice, Device> = HashMap::new();
    let mut validator = time::interval(Duration::from_millis(500));
    // Spawn the PnP handler..
    let (dev_sender, mut dev_receiver) = mpsc::channel(32);
    let sender_inner = dev_sender.clone();
    task::spawn(run_pnp_handler(dev_sender.clone()));

    loop {
        tokio::select! {
            Some(event) = dev_receiver.recv() => {
                match event {
                    DeviceEvents::Attached(goxlr_device) => {
                        info!("New Device Found: {:#?}", goxlr_device);

                        // Regardless of what happens below, we should note this is an active device..
                        if !device_list.contains(&goxlr_device) {
                            device_list.push(goxlr_device.clone());
                        }

                        // Create the Device..
                        let device = Device::new(goxlr_device.clone(), dev_sender.clone()).await;
                        match device {
                            Ok(device) => {
                                // Store this device as 'known'.
                                devices.insert(goxlr_device, device);
                            }
                            Err(e) => {
                                error!("Unable to Load GoXLR Device: {:?}", e);
                            }
                        }

                    },
                    DeviceEvents::Removed(device) => {
                        info!("Device Removed: {:#?}", device);

                        // Only attempt to stop this if it's active..
                        if devices.contains_key(&device) {
                            devices.get_mut(&device).unwrap().stop().await;
                            devices.remove(&device);
                        }

                        // Remove it from our list..
                        device_list.retain(|known_device| *known_device != device);
                    },
                    DeviceEvents::Error(device) => {
                        // If it's not been detached, we should just destroy the existing device
                        // and re-create it, with hopes it'll cleanly reset.
                        if devices.contains_key(&device) {
                            devices.get_mut(&device).unwrap().stop().await;
                            devices.remove(&device);
                        }

                        info!("Device Errored: {:#?}", device);
                    },
                }
            },
            _ = validator.tick() => {
                // This is primarily for situations where the device has been removed from the
                // hashmap, but hasn't been removed by the PnP handler (for example, on error)..
                for device in &device_list {
                    if !devices.contains_key(device) {
                        // Simply send a message to ourselves to attempt to reattach the GoXLR in
                        // question, and hope it can resume normal operation.
                        let _ = sender_inner.send(DeviceEvents::Attached(device.clone())).await;
                    }
                }
            }
        }
    }
}
