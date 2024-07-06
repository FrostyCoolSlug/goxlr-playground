use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
