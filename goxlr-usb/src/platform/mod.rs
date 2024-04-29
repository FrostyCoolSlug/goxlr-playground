use crate::common::command_handler::GoXLRCommands;
use crate::platform::common::device::{GoXLRConfiguration, GoXLRDevice};
use crate::USBLocation;
use anyhow::Result;

// This file will select which backend to use depending on platform, internally they'll all
// behave the same way.
pub mod common;

pub trait FullGoXLRDevice: GoXLRDevice + GoXLRCommands + Sync + Send {}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod tusb;
        use crate::platform::tusb::device;

        pub async fn find_devices() -> Vec<USBLocation> {
            crate::platform::tusb::pnp::get_devices()
        }

        pub async fn from_device(config: GoXLRConfiguration) -> Result<Box<dyn FullGoXLRDevice>> {
            device::TUSBAudioGoXLR::from_config(config).await
        }
    } else {
        mod libusb;

        pub async fn find_devices() -> Vec<USBLocation> {
            libusb::pnp::get_devices().await
        }

        pub async fn from_device(config: GoXLRConfiguration) -> Result<Box<dyn FullGoXLRDevice>> {
            libusb::device::LibUSBGoXLR::from_config(config).await
        }
    }
}
