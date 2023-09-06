use anyhow::{bail, Result};
use async_trait::async_trait;
use log::{debug, warn};
use strum::IntoEnumIterator;

use goxlr_shared::faders::Fader;
use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::components::fader::DeviceFader;
use crate::device::goxlr::device::GoXLR;

/// This trait is responsible for fader paging, anything that need to happen (including changing
/// the page) should be sent through methods here.
#[async_trait]
pub(crate) trait FaderPages {
    async fn load_current_page(&mut self, apply_states: bool) -> Result<()>;

    async fn next_page(&mut self) -> Result<()>;
    async fn prev_page(&mut self) -> Result<()>;

    async fn set_page(&mut self, page: usize) -> Result<()>;
}

#[async_trait]
impl FaderPages for GoXLR {
    /// Loads the current page based on the profile settings, this can be called after the page
    /// has been changed. 'apply_states' indicates whether we should immediately apply the colour
    /// scheme and button states.
    ///
    /// Normally you'll want to apply_states, unless you're loading a profile or changing other
    /// state / colour settings simultaneously to avoid unneeded calls.
    async fn load_current_page(&mut self, apply_states: bool) -> Result<()> {
        // Assign all the faders..
        let current = self.profile.pages.current;
        let faders = self.profile.pages.page_list[current].faders;
        for fader in Fader::iter() {
            self.assign_fader(fader, faders[fader]).await?;
        }

        if apply_states {
            let command = BasicResultCommand::SetColour(self.colour_scheme);
            self.send_no_result(command).await?;

            let command = BasicResultCommand::SetButtonStates(self.button_states);
            self.send_no_result(command).await?;
        }

        Ok(())
    }

    async fn next_page(&mut self) -> Result<()> {
        let page_count = self.profile.pages.page_list.len();
        let current = self.profile.pages.current;

        if page_count == 1 {
            warn!("Cannot change page, only one available.");
            return Ok(());
        }

        // Do we need to loop back to Page 1, or move ahead.
        let new_page = if current == page_count - 1 {
            0
        } else {
            current + 1
        };
        self.set_page(new_page).await
    }

    async fn prev_page(&mut self) -> Result<()> {
        let page_count = self.profile.pages.page_list.len();
        let current = self.profile.pages.current;

        if page_count == 1 {
            warn!("Cannot change page, only one available.");
            return Ok(());
        }

        // Do we need to loop back to Page 1, or move ahead.
        let new_page = if current == 0 {
            page_count - 1
        } else {
            current - 1
        };
        self.set_page(new_page).await
    }

    async fn set_page(&mut self, page: usize) -> Result<()> {
        let page_count = self.profile.pages.page_list.len();
        if page > page_count - 1 {
            bail!("Invalid Page Number: {}, Max: {}", page, page_count);
        }
        debug!("Changing Fader Page to {}", page);
        self.profile.pages.current = page;
        self.load_current_page(true).await
    }
}
