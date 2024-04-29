use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::channels::input::InputChannels;
use goxlr_shared::channels::output::OutputChannels;
use goxlr_shared::channels::volume::VolumeChannels;
use goxlr_shared::mute::ChannelMuteState;

/// While this technically matches FaderSources, it's imperative that this order is maintained, as
/// it's the order the GoXLR expects (hence why it's hidden away inside the 'USB' crate). In the
/// event something messages with the ordering of any things that map here, this will remain safe!
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ChannelList {
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

impl From<InputChannels> for ChannelList {
    fn from(value: InputChannels) -> Self {
        match value {
            InputChannels::Microphone => ChannelList::Microphone,
            InputChannels::Chat => ChannelList::Chat,
            InputChannels::Music => ChannelList::Music,
            InputChannels::Game => ChannelList::Game,
            InputChannels::Console => ChannelList::Console,
            InputChannels::LineIn => ChannelList::LineIn,
            InputChannels::System => ChannelList::System,
            InputChannels::Sample => ChannelList::Sample,
        }
    }
}

impl From<OutputChannels> for ChannelList {
    fn from(value: OutputChannels) -> Self {
        match value {
            OutputChannels::Headphones => ChannelList::Headphones,
            OutputChannels::ChatMic => ChannelList::Chat,
            OutputChannels::LineOut => ChannelList::LineOut,

            // Panicking isn't the best option here, but there shouldn't be attempts to
            // adjust the volume / assignment / mute state of channels which can't be adjusted!
            _ => panic!("Attempted to adjust a non-adjustable Channel!"),
        }
    }
}

impl From<FaderChannels> for ChannelList {
    fn from(value: FaderChannels) -> Self {
        match value {
            FaderChannels::Microphone => ChannelList::Microphone,
            FaderChannels::Chat => ChannelList::Chat,
            FaderChannels::Music => ChannelList::Music,
            FaderChannels::Game => ChannelList::Game,
            FaderChannels::Console => ChannelList::Console,
            FaderChannels::LineIn => ChannelList::LineIn,
            FaderChannels::System => ChannelList::System,
            FaderChannels::Sample => ChannelList::Sample,
            FaderChannels::Headphones => ChannelList::Headphones,
            FaderChannels::LineOut => ChannelList::LineOut,
        }
    }
}

impl From<VolumeChannels> for ChannelList {
    fn from(value: VolumeChannels) -> ChannelList {
        match value {
            VolumeChannels::MicrophoneMonitor => ChannelList::MicrophoneMonitor,
            VolumeChannels::Microphone => ChannelList::Microphone,
            VolumeChannels::Chat => ChannelList::Chat,
            VolumeChannels::Music => ChannelList::Music,
            VolumeChannels::Game => ChannelList::Game,
            VolumeChannels::Console => ChannelList::Console,
            VolumeChannels::LineIn => ChannelList::LineIn,
            VolumeChannels::System => ChannelList::System,
            VolumeChannels::Sample => ChannelList::Sample,
            VolumeChannels::Headphones => ChannelList::Headphones,
            VolumeChannels::LineOut => ChannelList::LineOut,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum MixOutputChannel {
    Headphones = 0x00,
    StreamMix = 0x01,
    LineOut = 0x02,
    ChatMic = 0x03,
    Sampler = 0x04,
}

impl From<OutputChannels> for MixOutputChannel {
    fn from(value: OutputChannels) -> Self {
        match value {
            OutputChannels::Headphones => MixOutputChannel::Headphones,
            OutputChannels::StreamMix => MixOutputChannel::StreamMix,
            OutputChannels::LineOut => MixOutputChannel::LineOut,
            OutputChannels::ChatMic => MixOutputChannel::ChatMic,
            OutputChannels::Sampler => MixOutputChannel::Sampler,
        }
    }
}
