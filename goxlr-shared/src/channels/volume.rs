#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::channels::fader::FaderChannels;

/// There are channels that have volume management
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
