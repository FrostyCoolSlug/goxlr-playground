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
pub enum RoutingInputChannel {
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

impl RoutingInputChannel {
    pub fn id(&self) -> u8 {
        match self {
            RoutingInputChannel::MicrophoneLeft => 0x02,
            RoutingInputChannel::MicrophoneRight => 0x03,
            RoutingInputChannel::MusicLeft => 0x0e,
            RoutingInputChannel::MusicRight => 0x0f,
            RoutingInputChannel::GameLeft => 0x0a,
            RoutingInputChannel::GameRight => 0x0b,
            RoutingInputChannel::ChatLeft => 0x0c,
            RoutingInputChannel::ChatRight => 0x0d,
            RoutingInputChannel::ConsoleLeft => 0x06,
            RoutingInputChannel::ConsoleRight => 0x07,
            RoutingInputChannel::LineInLeft => 0x04,
            RoutingInputChannel::LineInRight => 0x05,
            RoutingInputChannel::SystemLeft => 0x08,
            RoutingInputChannel::SystemRight => 0x09,
            RoutingInputChannel::SamplesLeft => 0x10,
            RoutingInputChannel::SamplesRight => 0x11,
        }
    }

    pub fn from_basic(basic: &InputChannels) -> (RoutingInputChannel, RoutingInputChannel) {
        match basic {
            InputChannels::Microphone => (
                RoutingInputChannel::MicrophoneLeft,
                RoutingInputChannel::MicrophoneRight,
            ),
            InputChannels::Chat => (
                RoutingInputChannel::ChatLeft,
                RoutingInputChannel::ChatRight,
            ),
            InputChannels::Music => (
                RoutingInputChannel::MusicLeft,
                RoutingInputChannel::MusicRight,
            ),
            InputChannels::Game => (
                RoutingInputChannel::GameLeft,
                RoutingInputChannel::GameRight,
            ),
            InputChannels::Console => (
                RoutingInputChannel::ConsoleLeft,
                RoutingInputChannel::ConsoleRight,
            ),
            InputChannels::LineIn => (
                RoutingInputChannel::LineInLeft,
                RoutingInputChannel::LineInRight,
            ),
            InputChannels::System => (
                RoutingInputChannel::SystemLeft,
                RoutingInputChannel::SystemRight,
            ),
            InputChannels::Sample => (
                RoutingInputChannel::SamplesLeft,
                RoutingInputChannel::SamplesRight,
            ),
        }
    }
}
