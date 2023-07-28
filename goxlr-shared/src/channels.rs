#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A list of channels classified as 'Inputs'
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
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
