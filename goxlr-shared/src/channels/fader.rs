use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::channels::volume::VolumeChannels;
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

impl Into<VolumeChannels> for FaderChannels {
    fn into(self) -> VolumeChannels {
        match self {
            FaderChannels::Microphone => VolumeChannels::Microphone,
            FaderChannels::Chat => VolumeChannels::Chat,
            FaderChannels::Music => VolumeChannels::Music,
            FaderChannels::Game => VolumeChannels::Game,
            FaderChannels::Console => VolumeChannels::Console,
            FaderChannels::LineIn => VolumeChannels::LineIn,
            FaderChannels::System => VolumeChannels::System,
            FaderChannels::Sample => VolumeChannels::Sample,
            FaderChannels::Headphones => VolumeChannels::Headphones,
            FaderChannels::LineOut => VolumeChannels::LineOut,
        }
    }
}
