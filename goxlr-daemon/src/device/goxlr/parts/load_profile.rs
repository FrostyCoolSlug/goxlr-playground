use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use strum::IntoEnumIterator;

use goxlr_profile::MuteState;
use goxlr_shared::buttons::{Buttons, InactiveButtonBehaviour};
use goxlr_shared::device::DeviceType;
use goxlr_shared::faders::{Fader, FaderSources};
use goxlr_shared::scribbles::Scribble;
use goxlr_shared::states::State;
use goxlr_usb_messaging::events::commands::BasicResultCommand::SetFaderStyle;
use goxlr_usb_messaging::events::commands::{BasicResultCommand, ChannelSource};

use crate::device::goxlr::goxlr::GoXLR;

/// This trait contains all methods needed to successfully load a profile, and are implemented
/// for the GoXLR type immediately after. This code assumes that self.profile is accurate.
#[async_trait]
pub(crate) trait LoadProfile {
    async fn load_profile(&mut self) -> Result<()>;
    async fn load_faders(&mut self) -> Result<()>;
    async fn load_volumes(&mut self) -> Result<()>;

    // Colour Related Commands
    async fn load_colours(&mut self) -> Result<()>;
    async fn load_button_states(&mut self) -> Result<()>;
    async fn load_display(&mut self) -> Result<()>;
}

#[async_trait]
impl LoadProfile for GoXLR {
    async fn load_profile(&mut self) -> Result<()> {
        self.load_faders().await?;
        self.load_colours().await?;

        Ok(())
    }

    async fn load_faders(&mut self) -> Result<()> {
        debug!("Assigning Faders..");
        let page = self.profile.pages.current;
        let faders = self.profile.pages.page_list[page].faders;
        for fader in Fader::iter() {
            let source = ChannelSource::FromFaderSource(faders[fader]);
            let message = BasicResultCommand::AssignFader(fader, source);
            self.send_no_result(message).await?;
        }

        Ok(())
    }

    async fn load_volumes(&mut self) -> Result<()> {
        debug!("Loading Volumes..");

        for channel in FaderSources::iter() {
            let volume = self.profile.channels[channel].volume;
            let target = ChannelSource::FromFaderSource(channel);

            let command = BasicResultCommand::SetVolume(target, volume);
            self.send_no_result(command).await?;
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
            let scheme = self.colour_scheme.get_fader_target(fader);
            scheme.replace(colours.into());

            // Set the Style for the fader..
            let display = channel.display.fader_display_mode.clone();
            self.send_no_result(SetFaderStyle(fader, display)).await?;

            // Get the button..
            let button = Buttons::from_fader(fader);

            // Get the Colour target for the button
            let target = button.into();
            let mute_colours = self.colour_scheme.get_two_colour_target(target);
            mute_colours.replace(channel.display.mute_colours.into());

            // Get the Screen colours for the fader..
            let scribble: Scribble = fader.into();
            let scribble = self.colour_scheme.get_two_colour_target(scribble.into());
            scribble.colour1 = channel.display.screen_display.colour;
        }

        let command = BasicResultCommand::SetColour(self.colour_scheme);
        self.send_no_result(command).await
    }

    async fn load_button_states(&mut self) -> Result<()> {
        // Get the Mute states from the faders..
        let fader_page = self.profile.pages.current;
        let faders = self.profile.pages.page_list[fader_page].faders;
        for fader in Fader::iter() {
            // Get the button..
            let button = Buttons::from_fader(fader);

            let channel = self.profile.channels[faders[fader]].clone();
            // Is this channel muted? If so, update the button..
            let mute_state = channel.mute_state;
            let mute_behaviour = channel.display.mute_colours.inactive_behaviour;

            // Get the Inactive behaviour..
            match mute_state {
                MuteState::Unmuted => {
                    // Apply 'Inactive' State..
                    self.button_states.set_state(
                        button,
                        match mute_behaviour {
                            InactiveButtonBehaviour::DimActive => State::DimmedColour1,
                            InactiveButtonBehaviour::DimInactive => State::DimmedColour2,
                            InactiveButtonBehaviour::InactiveColour => State::Colour2,
                        },
                    );
                }

                // This might need some more work..
                MuteState::MutedToTargets => self.button_states.set_state(button, State::Colour2),
                MuteState::MutedToAll => self.button_states.set_state(button, State::Blinking),
            }
        }

        let command = BasicResultCommand::SetButtonStates(self.button_states);
        self.send_no_result(command).await?;

        Ok(())
    }

    async fn load_display(&mut self) -> Result<()> {
        // If we're a Mini, nothing to do.
        if self.device.clone().unwrap().device_type == DeviceType::Mini {
            return Ok(());
        }

        Ok(())
    }
}
