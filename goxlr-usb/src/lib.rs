mod channels;
mod colours;
mod commands;
mod dcp;
pub mod platform;
pub mod pnp_base;

mod buttonstate;
mod encoders;
mod routing;
pub(crate) mod state_tracker;

// Definitions for the GoXLR PID / VID
pub const VID_GOXLR: u16 = 0x1220;
pub const PID_GOXLR_MINI: u16 = 0x8fe4;
pub const PID_GOXLR_FULL: u16 = 0x8fe0;

/// The definition of a GoXLR device, and it's location. Used in Device::new() to acquire and
/// setup the device handler.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GoXLRDevice {
    pub(crate) bus_number: u8,
    pub(crate) address: u8,
    pub(crate) identifier: Option<String>,
}
