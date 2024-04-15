use goxlr_shared::channels::MuteState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelCommand {
    SetVolume(u8),
    SetMute(MuteState),
}
