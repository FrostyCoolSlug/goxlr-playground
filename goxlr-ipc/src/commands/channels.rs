use goxlr_shared::channels::MuteState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelCommand {
    SetVolume(SetVolume),
    SetMute(SetMute),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetMute {
    pub mute_state: MuteState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVolume {
    pub volume: u8,
}
