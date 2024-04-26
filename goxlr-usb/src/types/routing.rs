use goxlr_shared::channels::input::InputChannels;
use goxlr_shared::channels::output::RoutingOutput;

#[derive(Copy, Clone, Debug)]
pub(crate) enum RoutingOutputDevice {
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

pub(crate) enum RoutingChannel {
    Left,
    Right,
}

impl RoutingOutputDevice {
    pub fn from(device: RoutingOutput, channel: RoutingChannel) -> RoutingOutputDevice {
        match device {
            RoutingOutput::Headphones => match channel {
                RoutingChannel::Left => RoutingOutputDevice::HeadphonesLeft,
                RoutingChannel::Right => RoutingOutputDevice::HeadphonesRight,
            },
            RoutingOutput::StreamMix => match channel {
                RoutingChannel::Left => RoutingOutputDevice::StreamMixLeft,
                RoutingChannel::Right => RoutingOutputDevice::StreamMixRight,
            },
            RoutingOutput::LineOut => match channel {
                RoutingChannel::Left => RoutingOutputDevice::LineOutLeft,
                RoutingChannel::Right => RoutingOutputDevice::LineOutRight,
            },
            RoutingOutput::ChatMic => match channel {
                RoutingChannel::Left => RoutingOutputDevice::ChatMicLeft,
                RoutingChannel::Right => RoutingOutputDevice::ChatMicRight,
            },
            RoutingOutput::Sampler => match channel {
                RoutingChannel::Left => RoutingOutputDevice::SamplerLeft,
                RoutingChannel::Right => RoutingOutputDevice::SamplerRight,
            },
            RoutingOutput::HardTune => RoutingOutputDevice::HardTune,
        }
    }
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
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum RoutingInputChannel {
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
    SampleRight,
    SampleLeft,
}

impl RoutingInputChannel {
    pub fn from(input: InputChannels, channel: RoutingChannel) -> Self {
        match input {
            InputChannels::Microphone => match channel {
                RoutingChannel::Left => RoutingInputChannel::MicrophoneLeft,
                RoutingChannel::Right => RoutingInputChannel::MicrophoneRight,
            },
            InputChannels::Chat => match channel {
                RoutingChannel::Left => RoutingInputChannel::ChatLeft,
                RoutingChannel::Right => RoutingInputChannel::ChatRight,
            },
            InputChannels::Music => match channel {
                RoutingChannel::Left => RoutingInputChannel::MusicLeft,
                RoutingChannel::Right => RoutingInputChannel::MusicRight,
            },
            InputChannels::Game => match channel {
                RoutingChannel::Left => RoutingInputChannel::GameLeft,
                RoutingChannel::Right => RoutingInputChannel::GameRight,
            },
            InputChannels::Console => match channel {
                RoutingChannel::Left => RoutingInputChannel::ConsoleLeft,
                RoutingChannel::Right => RoutingInputChannel::ConsoleRight,
            },
            InputChannels::LineIn => match channel {
                RoutingChannel::Left => RoutingInputChannel::LineInLeft,
                RoutingChannel::Right => RoutingInputChannel::LineInRight,
            },
            InputChannels::System => match channel {
                RoutingChannel::Left => RoutingInputChannel::SystemLeft,
                RoutingChannel::Right => RoutingInputChannel::SystemRight,
            },
            InputChannels::Sample => match channel {
                RoutingChannel::Left => RoutingInputChannel::SampleLeft,
                RoutingChannel::Right => RoutingInputChannel::SampleRight,
            },
        }
    }

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
            RoutingInputChannel::SampleLeft => 0x10,
            RoutingInputChannel::SampleRight => 0x11,
        }
    }
}
