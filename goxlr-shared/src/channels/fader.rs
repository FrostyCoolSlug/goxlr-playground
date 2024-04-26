use crate::channels::input::InputChannels;
#[cfg(feature = "clap")]
use clap::ValueEnum;
use enum_map::Enum;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Channels which can be assigned to Faders
#[derive(Debug, Copy, Clone, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum FaderChannels {
    Microphone,
    Chat,
    Music,
    Game,
    Console,
    LineIn,
    System,
    Sample,
    Headphones,
    LineOut,
}

/// A list of channels which can be assigned to a fader.
impl FaderChannels {
    pub fn has_sub_mix(&self) -> bool {
        if self == &FaderChannels::Headphones || self == &FaderChannels::LineOut {
            return false;
        }
        true
    }
}

impl From<InputChannels> for FaderChannels {
    fn from(value: InputChannels) -> Self {
        match value {
            InputChannels::Chat => FaderChannels::Chat,
            InputChannels::Music => FaderChannels::Music,
            InputChannels::Game => FaderChannels::Game,
            InputChannels::Console => FaderChannels::Console,
            InputChannels::LineIn => FaderChannels::LineIn,
            InputChannels::System => FaderChannels::System,
            InputChannels::Sample => FaderChannels::Sample,
            _ => panic!("Not a valid Fader source!"),
        }
    }
}
