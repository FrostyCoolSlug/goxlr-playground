use anyhow::Result;
use crate::common::command_handler::GoXLRCommands;
use crate::platform::common::device::{GoXLRConfiguration, GoXLRDevice};
use crate::USBLocation;

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
        
    }
}