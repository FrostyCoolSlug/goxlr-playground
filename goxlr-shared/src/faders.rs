use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A Simple list of the faders, with A being the far left
#[derive(Debug, Copy, Clone, Enum, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Fader {
    A,
    B,
    C,
    D,
}

/// A list of channels which can be assigned to a fader.
#[derive(Debug, Copy, Clone, Enum, EnumIter)]
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
