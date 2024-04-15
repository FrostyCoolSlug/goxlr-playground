use std::fmt::{Display, Formatter, write};

mod common;
mod goxlr;
mod platform;

pub(crate) mod types;

pub mod events;
pub mod handlers;
pub mod requests;
pub mod runners;

/// GoXLR USB Vendor ID
pub const VID_GOXLR: u16 = 0x1220;

/// GoXLR Product ID
pub const PID_GOXLR_FULL: u16 = 0x8fe0;

/// GoXLR Mini Product ID
pub const PID_GOXLR_MINI: u16 = 0x8fe4;

/// The location of a GoXLR device based on the USB bus
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct USBLocation {
    lib_usb: Option<LibUSB>,
    windows_usb: Option<WindowsUSB>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct LibUSB {
    pub(crate) bus_number: u8,
    pub(crate) address: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct WindowsUSB {
    pub(crate) identifier: String
}

pub struct DeviceHandle {
    handle: u32,
}

impl Display for USBLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(libusb) = &self.lib_usb {
            return write!(f, "[{}:{}]", libusb.bus_number, libusb.address);
        }
        if let Some(winusb) = &self.windows_usb {
            return write!(f, "[{}]", winusb.identifier);
        }
        write!(f, "[ERROR] Unknown Device identification")
    }
}
