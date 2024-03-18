/*
   An rusb implementation of the device finder, we simply attempt to Locate all GoXLR devices
   which are attached, and return the list with no extra work. This method works on both Windows
   and Linux
*/

use crate::{USBLocation, PID_GOXLR_FULL, PID_GOXLR_MINI, VID_GOXLR};

pub async fn get_devices() -> Vec<USBLocation> {
    let mut list = vec![];
    if let Ok(devices) = rusb::devices() {
        for device in devices.iter() {
            if let Ok(descriptor) = device.device_descriptor() {
                let bus_number = device.bus_number();
                let address = device.address();

                let vid = descriptor.vendor_id();
                let pid = descriptor.product_id();

                if vid == VID_GOXLR && (pid == PID_GOXLR_FULL || pid == PID_GOXLR_MINI) {
                    let device = USBLocation {
                        bus_number,
                        address,
                    };

                    list.push(device);
                }
            }
        }
    }
    list
}
