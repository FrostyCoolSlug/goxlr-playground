use strum::IntoEnumIterator;

use goxlr_shared::buttons::Buttons;
use goxlr_shared::faders::{Fader, FaderSources};

use crate::device::goxlr::device::GoXLR;

pub(crate) trait Profile {
    fn get_channel_for_button(&self, button: Buttons) -> FaderSources;
    fn get_button_for_channel(&self, channel: FaderSources) -> Option<Buttons>;
}

impl Profile for GoXLR {
    fn get_channel_for_button(&self, button: Buttons) -> FaderSources {
        let fader = Fader::from(button);
        let current_page = self.profile.pages.current;
        self.profile.pages.page_list[current_page].faders[fader]
    }

    fn get_button_for_channel(&self, channel: FaderSources) -> Option<Buttons> {
        let current_page = self.profile.pages.current;
        for fader in Fader::iter() {
            if self.profile.pages.page_list[current_page].faders[fader] == channel {
                return Some(Buttons::from_fader(fader));
            }
        }

        None
    }
}
