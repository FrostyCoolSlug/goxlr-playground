use crate::pnp_base::DeviceEvents;
use crate::{GoXLRDevice, PID_GOXLR_FULL, PID_GOXLR_MINI, VID_GOXLR};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

pub async fn run_pnp_handler(sender: Sender<DeviceEvents>) {
    // We siply look for GoXLRs and report on their state..
    let mut store: Vec<GoXLRDevice> = vec![];

    loop {
        let mut new_list = vec![];
        if let Ok(devices) = rusb::devices() {
            for device in devices.iter() {
                if let Ok(descriptor) = device.device_descriptor() {
                    let bus_number = device.bus_number();
                    let address = device.address();

                    if descriptor.vendor_id() == VID_GOXLR
                        && (descriptor.product_id() == PID_GOXLR_FULL
                            || descriptor.product_id() == PID_GOXLR_MINI)
                    {
                        let device = GoXLRDevice {
                            bus_number,
                            address,
                            identifier: None,
                        };

                        new_list.push(device);
                    }
                }
            }
        }

        // Compare the lists..
        for device in &new_list {
            if !store.contains(&device) {
                let _ = sender.send(DeviceEvents::Attached(device.clone())).await;
                store.push(device.clone());
            }
        }

        // We need to do this the boring way..
        let mut device_removed = false;
        for device in &store {
            if !new_list.contains(&device) {
                let _ = sender.send(DeviceEvents::Removed(device.clone())).await;
                device_removed = true;
            }
        }

        // We can't call .await inside a closure with a parameter, and we can't mutate a list
        // we're actively iterating.. So now do the cleanup if needed.
        // for devices that are gone.
        if device_removed {
            store.retain(|device| {
                if !new_list.contains(device) {
                    return false;
                }
                true
            });
        }

        sleep(Duration::from_millis(100)).await;
    }
}
