use anyhow::{bail, Result};
use async_trait::async_trait;
use goxlr_profile::FaderPage;
use log::{debug, warn};
use strum::IntoEnumIterator;

use goxlr_shared::faders::{Fader, FaderSources};
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

    // IPC Related Commands
    async fn add_page(&mut self) -> Result<()>;
    async fn remove_page(&mut self, page_number: usize) -> Result<()>;
    async fn set_page_fader_source(
        &mut self,
        page_number: usize,
        fader: Fader,
        channel: FaderSources,
    ) -> Result<()>;
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

    async fn add_page(&mut self) -> Result<()> {
        let page_count = self.profile.pages.page_list.len();
        if page_count >= 10 {
            bail!("Maximum 10 pages reached");
        }

        self.profile.pages.page_list.push(FaderPage::default());
        Ok(())
    }

    async fn remove_page(&mut self, page_number: usize) -> Result<()> {
        let page_count = self.profile.pages.page_list.len();
        if page_count == 1 {
            bail!("Unable to Remove Last Page");
        }

        if page_number > page_count - 1 {
            bail!("Invalid Page Number: {}, Max: {}", page_number, page_count);
        }

        self.profile.pages.page_list.remove(page_number);

        if page_number == self.profile.pages.current {
            // Move to the previous, or next, page..
            if page_number == 0 {
                self.set_page(0).await?;
            } else if page_number == page_count {
                self.set_page(page_number - 1).await?;
            } else {
                self.load_current_page(true).await?;
            }
        }

        Ok(())
    }

    async fn set_page_fader_source(
        &mut self,
        page_number: usize,
        fader: Fader,
        channel: FaderSources,
    ) -> Result<()> {
        let current_page_number = self.profile.pages.current;
        let page_count = self.profile.pages.page_list.len();

        if page_number > page_count - 1 {
            bail!("Invalid Page Number: {}, Max: {}", page_number, page_count);
        }

        // Is this channel already assigned to this page?
        let fader_page = &mut self.profile.pages.page_list[page_number].faders;
        for fader_iter in Fader::iter() {
            if fader_page[fader_iter] == channel {
                // We need to perform a swap with whatever is at the target fader..
                debug!("Switching Fader {:?} and {:?}", fader_iter, fader);
                fader_page[fader_iter] = fader_page[fader];
                break;
            }
        }
        fader_page[fader] = channel;
        debug!("{:?}", fader_page);

        if current_page_number == page_number {
            debug!("Reloading current page..");
            self.load_current_page(true).await?;
        }

        Ok(())
    }
}
