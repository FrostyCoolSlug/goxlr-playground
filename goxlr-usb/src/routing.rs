use goxlr_shared::channels::{InputChannels, OutputChannels};

#[derive(Copy, Clone, Debug)]
pub enum RoutingOutputDevice {
    HeadphonesRight,
    HeadphonesLeft,
    StreamMixRight,
    StreamMixLeft,
    ChatMicRight,
    ChatMicLeft,
    SamplerRight,
    SamplerLeft,
    LineOutRight,
    LineOutLeft,
    HardTune,
}

impl RoutingOutputDevice {
    pub fn position(&self) -> usize {
        match self {
            RoutingOutputDevice::HeadphonesLeft => 1,
            RoutingOutputDevice::HeadphonesRight => 3,
            RoutingOutputDevice::StreamMixLeft => 5,
            RoutingOutputDevice::StreamMixRight => 7,
            RoutingOutputDevice::ChatMicLeft => 9,
            RoutingOutputDevice::ChatMicRight => 11,
            RoutingOutputDevice::SamplerLeft => 13,
            RoutingOutputDevice::SamplerRight => 15,
            RoutingOutputDevice::LineOutLeft => 17,
            RoutingOutputDevice::LineOutRight => 19,
            RoutingOutputDevice::HardTune => 21,
        }
    }

    pub fn from_basic(basic: &OutputChannels) -> (RoutingOutputDevice, RoutingOutputDevice) {
        match basic {
            OutputChannels::Headphones => (
                RoutingOutputDevice::HeadphonesLeft,
                RoutingOutputDevice::HeadphonesRight,
            ),
            OutputChannels::StreamMix => (
                RoutingOutputDevice::StreamMixLeft,
                RoutingOutputDevice::StreamMixRight,
            ),
            OutputChannels::ChatMic => (
                RoutingOutputDevice::ChatMicLeft,
                RoutingOutputDevice::ChatMicRight,
            ),
            OutputChannels::Sampler => (
                RoutingOutputDevice::SamplerLeft,
                RoutingOutputDevice::SamplerRight,
            ),
            OutputChannels::LineOut => (
                RoutingOutputDevice::LineOutLeft,
                RoutingOutputDevice::LineOutRight,
            ),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RoutingInputDevice {
    MicrophoneRight,
    MicrophoneLeft,
    MusicRight,
    MusicLeft,
    GameRight,
    GameLeft,
    ChatRight,
    ChatLeft,
    ConsoleRight,
    ConsoleLeft,
    LineInRight,
    LineInLeft,
    SystemRight,
    SystemLeft,
    SamplesRight,
    SamplesLeft,
}

impl RoutingInputDevice {
    pub fn id(&self) -> u8 {
        match self {
            RoutingInputDevice::MicrophoneLeft => 0x02,
            RoutingInputDevice::MicrophoneRight => 0x03,
            RoutingInputDevice::MusicLeft => 0x0e,
            RoutingInputDevice::MusicRight => 0x0f,
            RoutingInputDevice::GameLeft => 0x0a,
            RoutingInputDevice::GameRight => 0x0b,
            RoutingInputDevice::ChatLeft => 0x0c,
            RoutingInputDevice::ChatRight => 0x0d,
            RoutingInputDevice::ConsoleLeft => 0x06,
            RoutingInputDevice::ConsoleRight => 0x07,
            RoutingInputDevice::LineInLeft => 0x04,
            RoutingInputDevice::LineInRight => 0x05,
            RoutingInputDevice::SystemLeft => 0x08,
            RoutingInputDevice::SystemRight => 0x09,
            RoutingInputDevice::SamplesLeft => 0x10,
            RoutingInputDevice::SamplesRight => 0x11,
        }
    }

    pub fn from_basic(basic: &InputChannels) -> (RoutingInputDevice, RoutingInputDevice) {
        match basic {
            InputChannels::Microphone => (
                RoutingInputDevice::MicrophoneLeft,
                RoutingInputDevice::MicrophoneRight,
            ),
            InputChannels::Chat => (RoutingInputDevice::ChatLeft, RoutingInputDevice::ChatRight),
            InputChannels::Music => (
                RoutingInputDevice::MusicLeft,
                RoutingInputDevice::MusicRight,
            ),
            InputChannels::Game => (RoutingInputDevice::GameLeft, RoutingInputDevice::GameRight),
            InputChannels::Console => (
                RoutingInputDevice::ConsoleLeft,
                RoutingInputDevice::ConsoleRight,
            ),
            InputChannels::LineIn => (
                RoutingInputDevice::LineInLeft,
                RoutingInputDevice::LineInRight,
            ),
            InputChannels::System => (
                RoutingInputDevice::SystemLeft,
                RoutingInputDevice::SystemRight,
            ),
            InputChannels::Sample => (
                RoutingInputDevice::SamplesLeft,
                RoutingInputDevice::SamplesRight,
            ),
        }
    }
}
