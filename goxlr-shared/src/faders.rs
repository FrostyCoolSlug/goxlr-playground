#[cfg(feature = "clap")]
use clap::ValueEnum;
use enum_map::Enum;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::buttons::Buttons;
use crate::interaction::InteractiveFaders;

/// A Simple list of the faders, with A being the far left
#[derive(Debug, Copy, Clone, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
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
