// Ok, so externally for this, we can simply use InputChannels enum, rather than having
// a second one to handle. We'll map it correctly here.

use goxlr_shared::channels::input::InputChannels;
use goxlr_shared::channels::sub_mix::SubMixChannels;
use goxlr_shared::submix::Mix;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SubMixChannelList {
    Microphone = 0x10,
    LineIn = 0x11,
    Console = 0x12,
    System = 0x13,
    Game = 0x14,
    Chat = 0x15,
    Sample = 0x16,
    Music = 0x17,
}

impl From<SubMixChannels> for SubMixChannelList {
    fn from(value: SubMixChannels) -> Self {
        match value {
            SubMixChannels::Microphone => SubMixChannelList::Microphone,
            SubMixChannels::Chat => SubMixChannelList::Chat,
            SubMixChannels::Music => SubMixChannelList::Music,
            SubMixChannels::Game => SubMixChannelList::Game,
            SubMixChannels::Console => SubMixChannelList::Console,
            SubMixChannels::LineIn => SubMixChannelList::LineIn,
            SubMixChannels::System => SubMixChannelList::System,
            SubMixChannels::Sample => SubMixChannelList::Sample,
        }
    }
}

impl From<InputChannels> for SubMixChannelList {
    fn from(value: InputChannels) -> Self {
        match value {
            InputChannels::Microphone => SubMixChannelList::Microphone,
            InputChannels::Chat => SubMixChannelList::Chat,
            InputChannels::Music => SubMixChannelList::Music,
            InputChannels::Game => SubMixChannelList::Game,
            InputChannels::Console => SubMixChannelList::Console,
            InputChannels::LineIn => SubMixChannelList::LineIn,
            InputChannels::System => SubMixChannelList::System,
            InputChannels::Sample => SubMixChannelList::Sample,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DeviceMix {
    A = 0x00,
    B = 0x01,
}

impl From<Mix> for DeviceMix {
    fn from(value: Mix) -> Self {
        match value {
            Mix::A => DeviceMix::A,
            Mix::B => DeviceMix::B,
        }
    }
}
