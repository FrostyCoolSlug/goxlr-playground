use anyhow::{Context, Result};
use goxlr_scribbles::get_scribble;
use log::debug;
use strum::IntoEnumIterator;

use goxlr_shared::buttons::Buttons;
use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::colours::Colour;
use goxlr_shared::device::{DeviceType, GoXLRFeature};
use goxlr_shared::faders::Fader;
use goxlr_shared::mute::MuteState;
use goxlr_shared::scribbles::Scribble;
use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::components::buttons::ButtonHandlers;
use crate::device::goxlr::components::load_profile::LoadProfile;
use crate::device::goxlr::components::mute_handler::MuteHandler;
use crate::device::goxlr::components::profile::Profile;
use crate::device::goxlr::device::GoXLR;

const SUBMIX_MITIGATION: &[FaderChannels] = &[FaderChannels::Headphones, FaderChannels::LineOut];

/// This trait is responsible for the management of faders, everything from the top of the
/// scribble display, to the bottom of the mute button. Any changes which are to occur to them
/// should make their way through here.
pub(crate) trait DeviceFader {
    async fn assign_fader(&mut self, fader: Fader, source: FaderChannels) -> Result<()>;
    async fn update_mute_state(&mut self, source: FaderChannels, state: MuteState) -> Result<()>;
}

impl DeviceFader for GoXLR {
    /// Ok, there are multiple steps when assigning a fader, we need to assign it, update the
    /// colours and the fader style, the scribble display, and the mute button state.
    ///
    /// Some settings may not need to be immediately applied (such as colours and mute state) as
    /// it makes more sense to apply them all at once if assigning multiple faders.
    async fn assign_fader(&mut self, fader: Fader, source: FaderChannels) -> Result<()> {
        debug!("Checking assign of {:?} to {:?}", source, fader);
        let assign = if let Some(state) = self.fader_state[fader] {
            if state != source {
                debug!("Change needed, assigning {:?} to {:?}", source, fader);
                true
            } else {
                debug!("{:?} already assigned to fader {:?}", source, fader);
                false
            }
        } else {
            debug!("Unknown State, Assigning {:?} to {:?}", source, fader);
            true
        };

        if !assign {
            return Ok(());
        }

        let command = BasicResultCommand::AssignFader(fader, source);
        self.send_no_result(command).await?;

        // Update the Cache after assignment
        self.fader_state[fader].replace(source);

        // Get the faders style
        let style = self.profile.channels.configs[source].display.clone();

        // Set the Faders Colour Style..
        let display_mode = style.fader_display_mode;
        debug!("Setting Fader {:?} display to {:?}", fader, display_mode);
        let command = BasicResultCommand::SetFaderStyle(fader, display_mode);
        self.send_no_result(command).await?;

        debug!("Colours: Screen, Fader and Mute Button for {:?}", fader);
        // Start setting up colours..
        self.set_fader_colours(source, false).await?;

        // While in the colour structure, scribbles have two colours, there's only one actually used.
        let scribble = Scribble::from(fader).into();
        let scribble_colour = self.colour_scheme.get_two_colour_target(scribble);
        scribble_colour.colour1 = style.screen_display.colour;

        // Set the colours for the mute button..
        let mute_button = Buttons::from_fader(fader);
        let mute_colours = self.colour_scheme.get_two_colour_target(mute_button.into());
        mute_colours.colour1 = style.mute_colours.active_colour;
        mute_colours.colour2 = style.mute_colours.inactive_colour;

        let device = self.device.as_ref().context("Device Not Found!")?;
        if device.device_type != DeviceType::Mini {
            let text = format!("{:?}", source);
            debug!("Setting Screen Text to {:?}", text);
            let scribble = get_scribble(None, Some(text), None, false);
            let command = BasicResultCommand::SetScribble(fader, scribble);
            self.send_no_result(command).await?;
        }

        // Get the button mute state for this channel..
        debug!("Loading Mute button state for {:?}", source);
        let state = self.get_mute_button_state(source);
        self.button_states.set_state(mute_button, state);

        Ok(())
    }

    /// Called when the mute state gets updated (probably from mute_handler.rs) to update the
    /// button and fader state.
    async fn update_mute_state(&mut self, source: FaderChannels, state: MuteState) -> Result<()> {
        self.profile.channels.configs[source].mute_state = state;
        if let Some(button) = self.get_button_for_channel(source) {
            let state = self.get_mute_button_state(source);
            self.button_states.set_state(button, state);
            self.apply_button_states().await?;
        }
        self.set_fader_colours(source, true).await
    }
}

trait DeviceFaderLocal {
    /// Gets the assigned fader for a source
    fn get_fader_for_channel(&mut self, source: FaderChannels) -> Option<Fader>;

    /// Updates colours for a fader if they don't match provided colours (true on change)
    fn update_colours(&mut self, c1: Colour, c2: Colour, current: Fader) -> bool;

    /// Applies fader colours based on config
    async fn set_fader_colours(&mut self, source: FaderChannels, apply: bool) -> Result<()>;
}

impl DeviceFaderLocal for GoXLR {
    fn get_fader_for_channel(&mut self, source: FaderChannels) -> Option<Fader> {
        let current_page = self.profile.pages.current;
        let current_page = &self.profile.pages.page_list[current_page];
        Fader::iter().find(|&fader| current_page.faders[fader] == source)
    }

    fn update_colours(&mut self, c1: Colour, c2: Colour, fader: Fader) -> bool {
        let current = self.colour_scheme.get_fader_target(fader);
        if current.colour1 != c1 || current.colour2 != c2 {
            // We need to refresh our faders
            current.colour1 = c1;
            current.colour2 = c2;

            return true;
        }
        false
    }

    async fn set_fader_colours(&mut self, source: FaderChannels, apply: bool) -> Result<()> {
        // Now we check whether we should dim the fader..
        if let Some(fader) = self.get_fader_for_channel(source) {
            if self.is_muted_to_all(source) {
                let dimmed = Colour::black();

                if self.update_colours(dimmed, dimmed, fader) && apply {
                    self.apply_colours().await?;
                }
            } else {
                // Get the original profile colours, and check if our map has them..
                let channel = &self.profile.channels.configs[source].display;

                let bottom = channel.fader_colours.bottom_colour;
                let top = channel.fader_colours.top_colour;

                if self.update_colours(top, bottom, fader) && apply {
                    self.apply_colours().await?;
                }
            }
        }
        Ok(())
    }
}
