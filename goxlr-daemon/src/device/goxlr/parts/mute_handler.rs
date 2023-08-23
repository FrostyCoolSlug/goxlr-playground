use crate::device::goxlr::device::GoXLR;
use anyhow::Result;
use goxlr_shared::channels::InputChannels;

pub(crate) trait MuteHandler {
    fn mute_channel_to_targets(&mut self) -> Result<MuteChanges>;
    fn mute_channel_to_all(&mut self) -> Result<MuteChanges>;
    fn unmute_channel(&mut self) -> Result<MuteChanges>;
}

impl MuteHandler for GoXLR {
    fn mute_channel_to_targets(&mut self) -> Result<MuteChanges> {
        Ok(Default::default())
    }

    fn mute_channel_to_all(&mut self) -> Result<MuteChanges> {
        Ok(Default::default())
    }

    fn unmute_channel(&mut self) -> Result<MuteChanges> {
        Ok(Default::default())
    }
}

/// This structure provides a list of things which have been changed by the mute commands,
/// generally speaking, they'll be followed up by applying them!
#[derive(Default)]
pub(crate) struct MuteChanges {
    button_state: bool,
    routing: Vec<InputChannels>,
}
