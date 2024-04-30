use crate::version::FirmwareVersions;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceInfo {
    pub serial: String,
    pub manufacture_date: String,

    pub device_type: DeviceType,
    pub firmware: FirmwareVersions,

    pub features: Vec<GoXLRFeature>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GoXLRFeature {
    Animation,
    SubMix,
    VoD,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceType {
    Full,
    Mini,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceColour {
    Black,
    White,
}
