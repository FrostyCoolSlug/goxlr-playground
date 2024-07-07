use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::channels::mute::MuteActionChannels;
#[cfg(feature = "clap")]
use clap::ValueEnum;

/// Channels which can be assigned to Faders
#[derive(Debug, Copy, Clone, Hash, Enum, EnumIter, Eq, PartialEq)]
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
