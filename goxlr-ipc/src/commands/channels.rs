use goxlr_shared::channels::MuteState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelCommand {
    SetVolume(SetVolume),
    SetMute(SetMute),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetMute {
    pub mute_state: MuteState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetVolume {
    pub volume: u8,
}
