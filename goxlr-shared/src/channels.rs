use crate::faders::FaderSources;
#[cfg(feature = "clap")]
use clap::ValueEnum;
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

/// This represents the current state of a Channel
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum MuteState {
    Unmuted,
    Pressed,
    Held,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ChannelMuteState {
    Muted,
    Unmuted,
}

impl From<FaderSources> for InputChannels {
    fn from(value: FaderSources) -> Self {
        match value {
            FaderSources::Microphone => InputChannels::Microphone,
            FaderSources::Chat => InputChannels::Chat,
            FaderSources::Music => InputChannels::Music,
            FaderSources::Game => InputChannels::Game,
            FaderSources::Console => InputChannels::Console,
            FaderSources::LineIn => InputChannels::LineIn,
            FaderSources::System => InputChannels::System,
            FaderSources::Sample => InputChannels::Sample,
            FaderSources::Headphones | FaderSources::LineOut | FaderSources::MicrophoneMonitor => {
                panic!("Invalid Mapping from FaderSources -> InputChannel")
            }
        }
    }
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
