use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::channels::fader::FaderChannels;
#[cfg(feature = "clap")]
use clap::ValueEnum;

#[derive(Debug, Copy, Clone, Hash, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum SubMixChannels {
    Microphone,
    Chat,
    Music,
    Game,
    Console,
    LineIn,
    System,
    Sample,
}

impl From<FaderChannels> for SubMixChannels {
    fn from(value: FaderChannels) -> Self {
        match value {
            FaderChannels::Microphone => SubMixChannels::Microphone,
            FaderChannels::Chat => SubMixChannels::Chat,
            FaderChannels::Music => SubMixChannels::Music,
            FaderChannels::Game => SubMixChannels::Game,
            FaderChannels::Console => SubMixChannels::Console,
            FaderChannels::LineIn => SubMixChannels::LineIn,
            FaderChannels::System => SubMixChannels::System,
            FaderChannels::Sample => SubMixChannels::Sample,
            _ => panic!("Attempted to look up Non-SubMix Channel: {:?}", value),
        }
    }
}
