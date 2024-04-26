use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "clap")]
use clap::ValueEnum;

#[derive(Debug, Copy, Clone, Hash, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum OutputChannels {
    Headphones,
    StreamMix,
    LineOut,
    ChatMic,
    Sampler,
}

#[derive(Debug, Copy, Clone, Hash, Enum, EnumIter, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
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
