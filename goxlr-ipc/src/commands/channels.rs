use goxlr_shared::channels::MuteState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelCommand {
    SetVolume(u8),
    SetMute(MuteState),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelResponse {
    Volume(u8),
    Mute(MuteState),
}
