use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::channels::fader::FaderChannels;
use crate::channels::CanFrom;
#[cfg(feature = "clap")]
use clap::ValueEnum;

/// Channels which can have custom Mute Actions
#[derive(Debug, Copy, Clone, Hash, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum MuteActionChannels {
    Microphone,
    Chat,
    Music,
    Game,
    Console,
    LineIn,
    System,
    Sample,
}

impl CanFrom<FaderChannels> for MuteActionChannels {
    fn can_from(value: FaderChannels) -> bool {
        match value {
            FaderChannels::Microphone => true,
            FaderChannels::Chat => true,
            FaderChannels::Music => true,
            FaderChannels::Game => true,
            FaderChannels::Console => true,
            FaderChannels::LineIn => true,
            FaderChannels::System => true,
            FaderChannels::Sample => true,
            FaderChannels::Headphones => false,
            FaderChannels::LineOut => false,
        }
    }
}

impl From<FaderChannels> for MuteActionChannels {
    fn from(value: FaderChannels) -> Self {
        match value {
            FaderChannels::Microphone => MuteActionChannels::Microphone,
            FaderChannels::Chat => MuteActionChannels::Chat,
            FaderChannels::Music => MuteActionChannels::Music,
            FaderChannels::Game => MuteActionChannels::Game,
            FaderChannels::Console => MuteActionChannels::Console,
            FaderChannels::LineIn => MuteActionChannels::LineIn,
            FaderChannels::System => MuteActionChannels::System,
            FaderChannels::Sample => MuteActionChannels::Sample,
            other => panic!("Cannot Cast from {:?} to MuteActionChannel", other),
        }
    }
}

impl From<MuteActionChannels> for FaderChannels {
    fn from(value: MuteActionChannels) -> Self {
        match value {
            MuteActionChannels::Microphone => FaderChannels::Microphone,
            MuteActionChannels::Chat => FaderChannels::Chat,
            MuteActionChannels::Music => FaderChannels::Music,
            MuteActionChannels::Game => FaderChannels::Game,
            MuteActionChannels::Console => FaderChannels::Console,
            MuteActionChannels::LineIn => FaderChannels::LineIn,
            MuteActionChannels::System => FaderChannels::System,
            MuteActionChannels::Sample => FaderChannels::Sample,
        }
    }
}
