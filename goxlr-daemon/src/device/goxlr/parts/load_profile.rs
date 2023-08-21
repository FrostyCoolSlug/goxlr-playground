use anyhow::Result;

use crate::device::goxlr::goxlr::GoXLR;
use async_trait::async_trait;
use goxlr_shared::faders::Fader;
use goxlr_usb_messaging::events::commands::BasicResultCommand::SetFaderStyle;
use goxlr_usb_messaging::events::commands::{BasicResultCommand, ChannelSource};
use strum::IntoEnumIterator;

/// This trait contains all methods needed to successfully load a profile, and are implemented
/// for the GoXLR type immediately after. This code assumes that self.profile is accurate.
#[async_trait]
pub(crate) trait LoadProfile {
    async fn load_profile(&mut self) -> Result<()>;
    async fn load_faders(&mut self) -> Result<()>;

    // Colour Related Commands
    async fn load_colours(&mut self) -> Result<()>;
}

#[async_trait]
impl LoadProfile for GoXLR {
    async fn load_profile(&mut self) -> Result<()> {
        self.load_faders().await?;
        self.load_colours().await?;

        Ok(())
    }

    async fn load_faders(&mut self) -> Result<()> {
        let page = self.profile.pages.current;
        let faders = self.profile.pages.page_list[page].faders;
        for fader in Fader::iter() {
            let source = ChannelSource::FromFaderSource(faders[fader]);
            let message = BasicResultCommand::AssignFader(fader, source);
            self.send_no_result(message).await?;
        }

        Ok(())
    }

    async fn load_colours(&mut self) -> Result<()> {
        // Pull the colour scheme from the profile..
        let fader_page = self.profile.pages.current;
        let faders = self.profile.pages.page_list[fader_page].faders;

        // Iterate the faders, pull and set the colour..
        for fader in Fader::iter() {
            let channel = self.profile.channels[faders[fader]].clone();

            let colours = channel.display.fader_colours;
            self.colour_scheme.set_fader_target(fader, colours.into());

            // Set the Style for the fader..
            let display = channel.display.fader_display_mode.clone();
            self.send_no_result(SetFaderStyle(fader, display)).await?;
        }

        let command = BasicResultCommand::SetColour(self.colour_scheme);
        self.send_no_result(command).await
    }
}
