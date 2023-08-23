use enum_map::Enum;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// A list of channels classified as 'Inputs'
#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InputChannels {
    Microphone,
    Chat,
    Music,
    Game,
    Console,
    LineIn,
    System,
    Sample,
}

/// A list of channels classified as 'Outputs'
#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OutputChannels {
    Headphones,
    StreamMix,
    LineOut,
    ChatMic,
    Sampler,
}

/// These are channels which simply have volume management
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VolumeChannels {
    MicrophoneMonitor,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RoutingOutput {
    Headphones,
    StreamMix,
    LineOut,
    ChatMic,
    Sampler,
    HardTune,
}

impl From<OutputChannels> for RoutingOutput {
    fn from(value: OutputChannels) -> Self {
        match value {
            OutputChannels::Headphones => RoutingOutput::Headphones,
            OutputChannels::StreamMix => RoutingOutput::StreamMix,
            OutputChannels::LineOut => RoutingOutput::LineOut,
            OutputChannels::ChatMic => RoutingOutput::ChatMic,
            OutputChannels::Sampler => RoutingOutput::Sampler,
        }
    }
}
