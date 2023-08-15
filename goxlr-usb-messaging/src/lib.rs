use std::fmt::{Display, Formatter};

mod platform;
pub mod runners;

/// GoXLR USB Vendor ID
pub const VID_GOXLR: u16 = 0x1220;

/// GoXLR Product ID
pub const PID_GOXLR_FULL: u16 = 0x8fe0;

/// GoXLR Mini Product ID
pub const PID_GOXLR_MINI: u16 = 0x8fe4;

/// The location of a GoXLR device based on the USB bus
#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct USBLocation {
    pub(crate) bus_number: u8,
    pub(crate) address: u8,
}

impl Display for USBLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.bus_number, self.address)
    }
}
