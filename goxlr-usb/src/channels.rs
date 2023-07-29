use goxlr_shared::channels::{InputChannels, OutputChannels, VolumeChannels};
use goxlr_shared::faders::FaderSources;

/// While this technically matches FaderSources, it's imperative that this order is maintained, as
/// it's the order the GoXLR expects (hence why it's hidden away inside the 'USB' crate). In the
/// event something messages with the ordering of any things that map here, this will remain safe!
enum AssignableChannel {
    Microphone,
    LineIn,
    Console,
    System,
    Game,
    Chat,
    Sample,
    Music,
    Headphones,
    MicrophoneMonitor,
    LineOut,
}

impl Into<AssignableChannel> for InputChannels {
    fn into(self) -> AssignableChannel {
        match self {
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

impl Into<AssignableChannel> for OutputChannels {
    fn into(self) -> AssignableChannel {
        match self {
            OutputChannels::Headphones => AssignableChannel::Headphones,
            OutputChannels::ChatMic => AssignableChannel::Chat,
            OutputChannels::LineOut => AssignableChannel::LineOut,

            // Panicking isn't the best option here, but there shouldn't be attempts to
            // adjust the volume / assignment / mute state of channels which can't be adjusted!
            _ => panic!("Attempted to adjust a non-adjustable Channel!"),
        }
    }
}

impl Into<AssignableChannel> for FaderSources {
    fn into(self) -> AssignableChannel {
        match self {
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

impl Into<AssignableChannel> for VolumeChannels {
    fn into(self) -> AssignableChannel {
        match self {
            VolumeChannels::MicrophoneMonitor => AssignableChannel::MicrophoneMonitor,
        }
    }
}
