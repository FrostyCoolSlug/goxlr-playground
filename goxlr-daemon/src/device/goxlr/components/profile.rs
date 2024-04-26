use strum::IntoEnumIterator;

use goxlr_shared::buttons::Buttons;
use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::faders::Fader;

use crate::device::goxlr::device::GoXLR;

pub(crate) trait Profile {
    fn get_channel_for_button(&self, button: Buttons) -> FaderChannels;
    fn get_button_for_channel(&self, channel: FaderChannels) -> Option<Buttons>;
}

impl Profile for GoXLR {
    fn get_channel_for_button(&self, button: Buttons) -> FaderChannels {
        let fader = Fader::from(button);
        let current_page = self.profile.pages.current;
        self.profile.pages.page_list[current_page].faders[fader]
    }

    fn get_button_for_channel(&self, channel: FaderChannels) -> Option<Buttons> {
        let current_page = self.profile.pages.current;
        for fader in Fader::iter() {
            if self.profile.pages.page_list[current_page].faders[fader] == channel {
                return Some(Buttons::from_fader(fader));
            }
        }

        None
    }
}
