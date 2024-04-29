use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::channels::fader::FaderChannels;
use crate::channels::volume::VolumeChannels;
use crate::channels::CanFrom;
#[cfg(feature = "clap")]
use clap::ValueEnum;

/// A list of All the GoXLR Channels
#[derive(Debug, Copy, Clone, Hash, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum AllChannels {
    Microphone,
    LineIn,
    Console,
    System,
    Game,
    Chat,
    Sample,
    Music,
    Headphones,
    MicrophoneMonitor,
    LineOut,
}

impl CanFrom<VolumeChannels> for AllChannels {
    fn can_from(_: VolumeChannels) -> bool {
        true
    }
}

impl From<VolumeChannels> for AllChannels {
    fn from(value: VolumeChannels) -> Self {
        match value {
            VolumeChannels::Microphone => AllChannels::Microphone,
            VolumeChannels::Chat => AllChannels::Chat,
            VolumeChannels::Music => AllChannels::Music,
            VolumeChannels::Game => AllChannels::Game,
            VolumeChannels::Console => AllChannels::Console,
            VolumeChannels::LineIn => AllChannels::LineIn,
            VolumeChannels::System => AllChannels::System,
            VolumeChannels::Sample => AllChannels::Sample,
            VolumeChannels::Headphones => AllChannels::Headphones,
            VolumeChannels::LineOut => AllChannels::LineOut,
            VolumeChannels::MicrophoneMonitor => AllChannels::MicrophoneMonitor,
        }
    }
}

impl CanFrom<FaderChannels> for AllChannels {
    fn can_from(_: FaderChannels) -> bool {
        true
    }
}

impl From<FaderChannels> for AllChannels {
    fn from(value: FaderChannels) -> Self {
        match value {
            FaderChannels::Microphone => AllChannels::Microphone,
            FaderChannels::Chat => AllChannels::Chat,
            FaderChannels::Music => AllChannels::Music,
            FaderChannels::Game => AllChannels::Game,
            FaderChannels::Console => AllChannels::Console,
            FaderChannels::LineIn => AllChannels::LineIn,
            FaderChannels::System => AllChannels::System,
            FaderChannels::Sample => AllChannels::Sample,
            FaderChannels::Headphones => AllChannels::Headphones,
            FaderChannels::LineOut => AllChannels::LineOut,
        }
    }
}
