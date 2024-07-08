use crate::device::goxlr::device::GoXLR;
use anyhow::Result;
use goxlr_shared::device::{DeviceInfo, GoXLRFeature};

pub(crate) mod buttons;
pub(crate) mod channel;
pub(crate) mod fader;
pub(crate) mod interactions;
pub(crate) mod load_profile;
pub(crate) mod mic;
pub(crate) mod mute_handler;
pub(crate) mod pages;
pub(crate) mod profile;
pub(crate) mod routing_handler;
pub(crate) mod submix;

pub fn has_feature(device: &Option<DeviceInfo>, feature: GoXLRFeature) -> Result<bool> {
    return Ok(device
        .as_ref()
        .expect("Missing Device")
        .features
        .contains(&feature));
}
