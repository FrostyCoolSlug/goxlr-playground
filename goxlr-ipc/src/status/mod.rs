mod device;
mod mic;

use goxlr_profile::{MicProfile, Profile};
use goxlr_shared::device::DeviceInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub hardware: DeviceInfo,
    pub serial: String,
    pub config: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub device: Profile,
    pub mic_profile: MicProfile,
}
