use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::channels::fader::FaderChannels;

use crate::channels::sub_mix::SubMixChannels;
use crate::channels::CanFrom;
#[cfg(feature = "clap")]
use clap::ValueEnum;

/// There are channels that have volume management
#[derive(Debug, Copy, Clone, Hash, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum VolumeChannels {
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

impl CanFrom<FaderChannels> for VolumeChannels {
    fn can_from(_: FaderChannels) -> bool {
        // All FaderChannels can be Mapped to VolumeChannels
        true
    }
}

impl From<FaderChannels> for VolumeChannels {
    fn from(value: FaderChannels) -> Self {
        match value {
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

impl CanFrom<SubMixChannels> for VolumeChannels {
    fn can_from(_: SubMixChannels) -> bool {
        true
    }
}

impl From<SubMixChannels> for VolumeChannels {
    fn from(value: SubMixChannels) -> Self {
        match value {
            SubMixChannels::Microphone => VolumeChannels::Microphone,
            SubMixChannels::Chat => VolumeChannels::Chat,
            SubMixChannels::Music => VolumeChannels::Music,
            SubMixChannels::Game => VolumeChannels::Game,
            SubMixChannels::Console => VolumeChannels::Console,
            SubMixChannels::LineIn => VolumeChannels::LineIn,
            SubMixChannels::System => VolumeChannels::System,
            SubMixChannels::Sample => VolumeChannels::Sample,
        }
    }
}
