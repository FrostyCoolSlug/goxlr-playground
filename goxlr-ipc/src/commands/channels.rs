use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::channels::sub_mix::SubMixChannels;
use goxlr_shared::channels::volume::VolumeChannels;
use goxlr_shared::mute::MuteState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelCommands {
    Volume(ChannelVolume),
    Mute(MuteCommand),
    SubMix(SubMix),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelVolume {
    pub channel: VolumeChannels,
    pub volume: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuteCommand {
    pub channel: FaderChannels,
    pub state: MuteState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubMix {
    pub channel: SubMixChannels,
    pub command: SubMixCommands,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubMixCommands {
    Volume(u8),
    Linked(bool),
}
