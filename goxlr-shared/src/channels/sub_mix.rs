use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::channels::fader::FaderChannels;
use crate::channels::CanFrom;

use crate::channels::volume::VolumeChannels;
#[cfg(feature = "clap")]
use clap::ValueEnum;

#[derive(Debug, Copy, Clone, Hash, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum SubMixChannels {
    Microphone,
    Chat,
    Music,
    Game,
    Console,
    LineIn,
    System,
    Sample,
}

impl CanFrom<VolumeChannels> for SubMixChannels {
    fn can_from(value: VolumeChannels) -> bool {
        matches!(
            value,
            VolumeChannels::Microphone
                | VolumeChannels::Chat
                | VolumeChannels::Music
                | VolumeChannels::Game
                | VolumeChannels::Console
                | VolumeChannels::LineIn
                | VolumeChannels::System
                | VolumeChannels::Sample
        )
    }
}

impl From<VolumeChannels> for SubMixChannels {
    fn from(value: VolumeChannels) -> Self {
        match value {
            VolumeChannels::Microphone => SubMixChannels::Microphone,
            VolumeChannels::Chat => SubMixChannels::Chat,
            VolumeChannels::Music => SubMixChannels::Music,
            VolumeChannels::Game => SubMixChannels::Game,
            VolumeChannels::Console => SubMixChannels::Console,
            VolumeChannels::LineIn => SubMixChannels::LineIn,
            VolumeChannels::System => SubMixChannels::System,
            VolumeChannels::Sample => SubMixChannels::Sample,
            _ => panic!("Attempted to look up Non-SubMix Channel: {:?}", value),
        }
    }
}

impl CanFrom<FaderChannels> for SubMixChannels {
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

impl From<FaderChannels> for SubMixChannels {
    fn from(value: FaderChannels) -> Self {
        match value {
            FaderChannels::Microphone => SubMixChannels::Microphone,
            FaderChannels::Chat => SubMixChannels::Chat,
            FaderChannels::Music => SubMixChannels::Music,
            FaderChannels::Game => SubMixChannels::Game,
            FaderChannels::Console => SubMixChannels::Console,
            FaderChannels::LineIn => SubMixChannels::LineIn,
            FaderChannels::System => SubMixChannels::System,
            FaderChannels::Sample => SubMixChannels::Sample,
            _ => panic!("Attempted to look up Non-SubMix Channel: {:?}", value),
        }
    }
}
