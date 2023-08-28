use enum_map::Enum;
use strum::EnumIter;

use crate::buttons::Buttons;
use crate::channels::InputChannels;
use crate::interaction::InteractiveFaders;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A Simple list of the faders, with A being the far left
#[derive(Debug, Copy, Clone, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Fader {
    A,
    B,
    C,
    D,
}

impl From<InteractiveFaders> for Fader {
    fn from(value: InteractiveFaders) -> Self {
        match value {
            InteractiveFaders::A => Fader::A,
            InteractiveFaders::B => Fader::B,
            InteractiveFaders::C => Fader::C,
            InteractiveFaders::D => Fader::D,
        }
    }
}

impl From<Buttons> for Fader {
    fn from(value: Buttons) -> Self {
        match value {
            Buttons::FaderA => Fader::A,
            Buttons::FaderB => Fader::B,
            Buttons::FaderC => Fader::C,
            Buttons::FaderD => Fader::D,
            _ => {
                panic!("Button isn't attached to a fader!");
            }
        }
    }
}

/// A list of channels which can be assigned to a fader.
#[derive(Debug, Copy, Clone, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FaderSources {
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
    MicrophoneMonitor,
}

impl From<InputChannels> for FaderSources {
    fn from(value: InputChannels) -> Self {
        match value {
            InputChannels::Microphone => FaderSources::Microphone,
            InputChannels::Chat => FaderSources::Chat,
            InputChannels::Music => FaderSources::Music,
            InputChannels::Game => FaderSources::Game,
            InputChannels::Console => FaderSources::Console,
            InputChannels::LineIn => FaderSources::LineIn,
            InputChannels::System => FaderSources::System,
            InputChannels::Sample => FaderSources::Sample,
        }
    }
}
