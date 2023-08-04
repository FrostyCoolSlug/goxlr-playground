use goxlr_shared::interaction::{InteractiveButtons, InteractiveEncoders, InteractiveFaders};

pub mod commands;
pub mod platform;
pub mod pnp_base;

pub(crate) mod state_tracker;

mod button_state;
mod channels;
mod colours;
mod dcp;
mod encoders;
mod goxlr_commands;
mod routing;

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

pub enum ChangeEvent {
    ButtonDown(InteractiveButtons),
    ButtonUp(InteractiveButtons),
    VolumeChange(InteractiveFaders, u8),
    EncoderChange(InteractiveEncoders, i8),
}
