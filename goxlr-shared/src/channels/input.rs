use crate::channels::fader::FaderChannels;
use enum_map::Enum;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
            FaderChannels::Headphones | FaderChannels::LineOut => {
                panic!("Invalid Mapping from FaderSources -> InputChannel")
            }
        }
    }
}
