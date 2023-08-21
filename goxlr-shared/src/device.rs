use crate::version::FirmwareVersions;

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub serial: String,
    pub manufacture_date: String,

    pub device_type: DeviceType,
    pub firmware: FirmwareVersions,

    pub features: Vec<GoXLRFeature>,
}

#[derive(Debug, Clone)]
pub enum GoXLRFeature {
    Animation,
    Submix,
    VoD,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeviceType {
    Full,
    Mini,
}
