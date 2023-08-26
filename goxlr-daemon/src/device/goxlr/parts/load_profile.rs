use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use strum::IntoEnumIterator;

use goxlr_profile::MuteState;
use goxlr_shared::buttons::{Buttons, InactiveButtonBehaviour};
use goxlr_shared::channels::{InputChannels, OutputChannels, RoutingOutput};
use goxlr_shared::device::DeviceType;
use goxlr_shared::faders::{Fader, FaderSources};
use goxlr_shared::routing::RouteValue;
use goxlr_shared::scribbles::Scribble;
use goxlr_shared::states::State;
use goxlr_usb_messaging::events::commands::BasicResultCommand::SetFaderStyle;
use goxlr_usb_messaging::events::commands::{BasicResultCommand, ChannelSource};

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::parts::mute_handler::MuteHandler;

/// This trait contains all methods needed to successfully load a profile, and are implemented
/// for the GoXLR type immediately after. This code assumes that self.profile is accurate.
#[async_trait]
pub(crate) trait LoadProfile {
    async fn load_profile(&mut self) -> Result<()>;
    async fn load_faders(&mut self) -> Result<()>;
    async fn load_volumes(&mut self) -> Result<()>;
    async fn load_mute_states(&mut self) -> Result<()>;

    // Colour Related Commands
    async fn load_colours(&mut self) -> Result<()>;
    async fn load_fader_display(&mut self) -> Result<()>;

    //async fn load_button_states(&mut self) -> Result<()>;
    async fn load_display(&mut self) -> Result<()>;

    // Routing Commands
    async fn setup_routing(&mut self) -> Result<()>;
    async fn apply_routing(&mut self) -> Result<()>;

    // Button States..
    async fn setup_button_states(&mut self) -> Result<()>;
    async fn apply_button_states(&mut self) -> Result<()>;
}

#[async_trait]
impl LoadProfile for GoXLR {
    async fn load_profile(&mut self) -> Result<()> {
        debug!("Beginning Profile Load");
        // These are setup methods, to do any pre-profile handling and setup..
        self.setup_routing().await?;
        self.setup_button_states().await?;

        // Go through the profile components and apply them to the GoXLR
        self.load_faders().await?;
        self.load_volumes().await?;

        self.load_colours().await?;
        self.load_fader_display().await?;

        // Finalise things setup earlier
        self.apply_routing().await?;
        self.apply_button_states().await?;

        debug!("Completed Profile Load");
        Ok(())
    }

    async fn load_faders(&mut self) -> Result<()> {
        debug!("Assigning Faders..");
        let page = self.profile.pages.current;
        let faders = self.profile.pages.page_list[page].faders;
        for fader in Fader::iter() {
            debug!("Assigning Fader {:?} to {:?}", fader, faders[fader]);

            if let Some(channel) = self.fader_state[fader] {
                if channel == faders[fader] {
                    debug!("Fader {:?} already assigned to {:?}", fader, faders[fader]);
                    continue;
                }
            }

            let source = ChannelSource::FromFaderSource(faders[fader]);
            let message = BasicResultCommand::AssignFader(fader, source);
            self.send_no_result(message).await?;

            // Replace our Cached Version..
            self.fader_state[fader].replace(faders[fader]);
        }

        Ok(())
    }

    async fn load_volumes(&mut self) -> Result<()> {
        debug!("Loading Volumes..");

        for channel in FaderSources::iter() {
            let volume = self.profile.channels[channel].volume;
            let target = ChannelSource::FromFaderSource(channel);

            debug!(
                "Setting Volume for {:?} from profile to {:?}",
                channel, volume
            );

            let command = BasicResultCommand::SetVolume(target, volume);
            self.send_no_result(command).await?;
        }

        Ok(())
    }

    async fn load_mute_states(&mut self) -> Result<()> {
        debug!("Loading Mute States");

        for source in FaderSources::iter() {
            let state = self.profile.channels[source].mute_state;
            let channel = self.profile.channels[source].clone();

            match state {
                MuteState::Unmuted => self.unmute(source).await?,
                MuteState::Pressed | MuteState::Held => {
                    let actions = channel.mute_actions[state.into()].clone();

                    // We can ignore the return value of this as we're loading a profile, we're
                    // going to forcefully apply the routing and states later.
                    self.mute_to_target(source, actions.mute_targets).await?
                }
            };
        }
        Ok(())
    }

    async fn load_colours(&mut self) -> Result<()> {
        debug!("Loading Colour Map..");

        // Pull the colour scheme from the profile..
        let fader_page = self.profile.pages.current;
        let faders = self.profile.pages.page_list[fader_page].faders;

        // Iterate the faders, pull and set the colour..
        for fader in Fader::iter() {
            let channel = self.profile.channels[faders[fader]].clone();

            let colours = channel.display.fader_colours;
            let scheme = self.colour_scheme.get_fader_target(fader);
            scheme.replace(colours.into());

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

    async fn load_fader_display(&mut self) -> Result<()> {
        let fader_page = self.profile.pages.current;
        let faders = self.profile.pages.page_list[fader_page].faders;

        for fader in Fader::iter() {
            let source = faders[fader];
            let channel = self.profile.channels[source].clone();
            let display = channel.display.fader_display_mode;

            // Set the Style for the fader..
            self.send_no_result(SetFaderStyle(fader, display)).await?;
        }

        Ok(())
    }

    async fn load_display(&mut self) -> Result<()> {
        // If we're a Mini, nothing to do.
        if self.device.clone().unwrap().device_type == DeviceType::Mini {
            return Ok(());
        }

        Ok(())
    }

    async fn setup_routing(&mut self) -> Result<()> {
        // Nothing to do here yet, we defer to the GoXLR struct to handle setup..
        Ok(())
    }

    async fn apply_routing(&mut self) -> Result<()> {
        // Once we reach here, all routing changes should have been setup, so we apply routing
        // for all input channels.

        for channel in InputChannels::iter() {
            let routes = self.routing_state.get_input_routes(channel);
            let command = BasicResultCommand::ApplyRouting(channel, routes);
            self.send_no_result(command).await?;
        }
        Ok(())
    }

    async fn setup_button_states(&mut self) -> Result<()> {
        // By default, all states are 'inactive' (DimmedColour1)
        debug!("Resetting Button States");
        self.button_states = Default::default();

        debug!("Building Initial States..");
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
                MuteState::Pressed => self.button_states.set_state(button, State::Colour2),
                MuteState::Held => self.button_states.set_state(button, State::Blinking),
            }
        }

        Ok(())
    }

    async fn apply_button_states(&mut self) -> Result<()> {
        let command = BasicResultCommand::SetButtonStates(self.button_states);
        self.send_no_result(command).await?;

        Ok(())
    }
}
