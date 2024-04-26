use goxlr_shared::mute::MuteState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelCommand {
    Volume(u8),
    SubVolume(u8),
    Mute(MuteState),
    SubMixLinked(bool),
}
