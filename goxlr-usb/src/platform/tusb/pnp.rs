use crate::platform::tusb::tusbaudio::TUSB_INTERFACE;
use crate::{USBLocation, WindowsUSB};

pub fn get_devices() -> Vec<USBLocation> {
    let _ = TUSB_INTERFACE.spawn_pnp_handle_rusb();
    let mut list = Vec::new();

    // Ok, this is slightly different now..
    let devices = TUSB_INTERFACE.get_devices();
    for device in devices {
        list.push(USBLocation {
            lib_usb: None,
            windows_usb: Some(WindowsUSB {
                identifier: device
            })
        })
    }
    list
}