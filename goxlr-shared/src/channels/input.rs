use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::channels::fader::FaderChannels;
use crate::channels::CanFrom;
#[cfg(feature = "clap")]
use clap::ValueEnum;

#[derive(Debug, Copy, Clone, Hash, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum InputChannels {
    Microphone,
    Chat,
    Music,
    Game,
    Console,
    LineIn,
    System,
    Sample,
}

impl CanFrom<FaderChannels> for InputChannels {
    fn can_from(value: FaderChannels) -> bool {
        matches!(
            value,
            FaderChannels::Microphone
                | FaderChannels::Chat
                | FaderChannels::Music
                | FaderChannels::Game
                | FaderChannels::Console
                | FaderChannels::LineIn
                | FaderChannels::System
                | FaderChannels::Sample
        )
    }
}

impl From<FaderChannels> for InputChannels {
    fn from(value: FaderChannels) -> Self {
        match value {
            FaderChannels::Microphone => InputChannels::Microphone,
            FaderChannels::Chat => InputChannels::Chat,
            FaderChannels::Music => InputChannels::Music,
            FaderChannels::Game => InputChannels::Game,
            FaderChannels::Console => InputChannels::Console,
            FaderChannels::LineIn => InputChannels::LineIn,
            FaderChannels::System => InputChannels::System,
            FaderChannels::Sample => InputChannels::Sample,
            _ => panic!("Attempted to Map a Non-input channel: {:?}", value),
        }
    }
}
