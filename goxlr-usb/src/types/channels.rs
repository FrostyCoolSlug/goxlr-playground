use goxlr_shared::channels::{ChannelMuteState, InputChannels, OutputChannels, VolumeChannels};
use goxlr_shared::faders::FaderSources;

/// While this technically matches FaderSources, it's imperative that this order is maintained, as
/// it's the order the GoXLR expects (hence why it's hidden away inside the 'USB' crate). In the
/// event something messages with the ordering of any things that map here, this will remain safe!
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum AssignableChannel {
    Microphone = 0x00,
    LineIn = 0x01,
    Console = 0x02,
    System = 0x03,
    Game = 0x04,
    Chat = 0x05,
    Sample = 0x06,
    Music = 0x07,
    Headphones = 0x08,
    MicrophoneMonitor = 0x09,
    LineOut = 0x0A,
}

impl From<InputChannels> for AssignableChannel {
    fn from(value: InputChannels) -> Self {
        match value {
            InputChannels::Microphone => AssignableChannel::Microphone,
            InputChannels::Chat => AssignableChannel::Chat,
            InputChannels::Music => AssignableChannel::Music,
            InputChannels::Game => AssignableChannel::Game,
            InputChannels::Console => AssignableChannel::Console,
            InputChannels::LineIn => AssignableChannel::LineIn,
            InputChannels::System => AssignableChannel::System,
            InputChannels::Sample => AssignableChannel::Sample,
        }
    }
}

impl From<OutputChannels> for AssignableChannel {
    fn from(value: OutputChannels) -> Self {
        match value {
            OutputChannels::Headphones => AssignableChannel::Headphones,
            OutputChannels::ChatMic => AssignableChannel::Chat,
            OutputChannels::LineOut => AssignableChannel::LineOut,

            // Panicking isn't the best option here, but there shouldn't be attempts to
            // adjust the volume / assignment / mute state of channels which can't be adjusted!
            _ => panic!("Attempted to adjust a non-adjustable Channel!"),
        }
    }
}

impl From<FaderSources> for AssignableChannel {
    fn from(value: FaderSources) -> Self {
        match value {
            FaderSources::Microphone => AssignableChannel::Microphone,
            FaderSources::Chat => AssignableChannel::Chat,
            FaderSources::Music => AssignableChannel::Music,
            FaderSources::Game => AssignableChannel::Game,
            FaderSources::Console => AssignableChannel::Console,
            FaderSources::LineIn => AssignableChannel::LineIn,
            FaderSources::System => AssignableChannel::System,
            FaderSources::Sample => AssignableChannel::Sample,
            FaderSources::Headphones => AssignableChannel::Headphones,
            FaderSources::LineOut => AssignableChannel::LineOut,
            FaderSources::MicrophoneMonitor => AssignableChannel::MicrophoneMonitor,
        }
    }
}

impl From<VolumeChannels> for AssignableChannel {
    fn from(value: VolumeChannels) -> AssignableChannel {
        match value {
            VolumeChannels::MicrophoneMonitor => AssignableChannel::MicrophoneMonitor,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ChannelState {
    Unmuted = 0x00,
    Muted = 0x01,
}

impl From<ChannelMuteState> for ChannelState {
    fn from(value: ChannelMuteState) -> Self {
        match value {
            ChannelMuteState::Muted => ChannelState::Muted,
            ChannelMuteState::Unmuted => ChannelState::Unmuted,
        }
    }
}
