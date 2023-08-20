// Ok, so externally for this, we can simply use InputChannels enum, rather than having
// a second one to handle. We'll map it correctly here.

use goxlr_shared::channels::InputChannels;
use goxlr_shared::submix::Mix;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SubMixChannelName {
    Microphone = 0x10,
    LineIn = 0x11,
    Console = 0x12,
    System = 0x13,
    Game = 0x14,
    Chat = 0x15,
    Sample = 0x16,
    Music = 0x17,
}

impl From<InputChannels> for SubMixChannelName {
    fn from(value: InputChannels) -> Self {
        match value {
            InputChannels::Microphone => SubMixChannelName::Microphone,
            InputChannels::Chat => SubMixChannelName::Chat,
            InputChannels::Music => SubMixChannelName::Music,
            InputChannels::Game => SubMixChannelName::Game,
            InputChannels::Console => SubMixChannelName::Console,
            InputChannels::LineIn => SubMixChannelName::LineIn,
            InputChannels::System => SubMixChannelName::System,
            InputChannels::Sample => SubMixChannelName::Sample,
        }
    }
}

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
