use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use strum::IntoEnumIterator;

use goxlr_profile::MuteState;
use goxlr_shared::channels::{InputChannels, OutputChannels, RoutingOutput};
use goxlr_shared::colours::TwoColourTargets;
use goxlr_shared::faders::FaderSources;
use goxlr_shared::routing::RouteValue;
use goxlr_usb_messaging::events::commands::{BasicResultCommand, ChannelSource};

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::parts::buttons::ButtonHandlers;
use crate::device::goxlr::parts::mute_handler::MuteHandler;
use crate::device::goxlr::parts::pages::FaderPages;
use crate::device::goxlr::parts::routing_handler::RoutingHandler;

/// This trait contains all public methods needed to successfully load a profile, and are implemented
/// for the GoXLR type immediately after. This code assumes that self.profile is accurate.
#[async_trait]
pub(crate) trait LoadProfile {
    async fn load_profile(&mut self) -> Result<()>;
}

#[async_trait]
impl LoadProfile for GoXLR {
    async fn load_profile(&mut self) -> Result<()> {
        debug!("Beginning Profile Load");
        // These are setup methods, to do any pre-profile handling and setup..
        self.setup_routing();
        self.setup_button_states();
        self.setup_colours();

        // Go through the profile components and apply them to the GoXLR
        self.load_volumes().await?;
        self.load_current_page(false).await?;

        // Finalise things setup earlier
        self.load_colours().await?;
        self.apply_routing().await?;
        self.apply_button_states().await?;

        debug!("Completed Profile Load");
        Ok(())
    }
}

/// This trait contains methods which are local to this mod. Traits require an attached scope to
/// make functions available to other classes, but we should limit that level of communication only
/// to things which should be exposed.
#[async_trait]
trait LoadProfileLocal {
    /// These first three functions are for base setup, creating the scheme or the settings
    /// prior to actually doing any of the loading.
    fn setup_routing(&mut self);
    fn setup_colours(&mut self);
    fn setup_button_states(&mut self);

    /// The next three are responsible for loading the various parts of the device..
    async fn load_volumes(&self) -> Result<()>;
    async fn load_mute_states(&mut self) -> Result<()>;
    async fn load_colours(&mut self) -> Result<()>;

    /// And finally, apply anything that's been configured above
    async fn apply_routing(&self) -> Result<()>;
}

#[async_trait]
impl LoadProfileLocal for GoXLR {
    fn setup_routing(&mut self) {
        debug!("Loading Routing from Profile: ");
        debug!("Routing Table: {:#?}", self.profile.routing);

        for channel in InputChannels::iter() {
            for output in OutputChannels::iter() {
                let value = match self.profile.routing[channel][output] {
                    true => RouteValue::On,
                    false => RouteValue::Off,
                };

                let output = RoutingOutput::from(output);

                // Set routing will return true / false if the route was actually changed, because
                // we're loading this from a profile, we don't need to worry about that, as all
                // routing will be updated at the end of the load.
                self.routing_state.set_routing(channel, output, value);
            }
        }
    }

    fn setup_colours(&mut self) {
        debug!("Initialising Colour Map..");
        self.colour_scheme = Default::default();
    }

    fn setup_button_states(&mut self) {
        // By default, all states are 'inactive' (DimmedColour1)
        debug!("Resetting Button States");
        self.button_states = Default::default();

        debug!("Building Initial States..");
        // Fader Mute buttons are handled by fader.rs
    }

    async fn load_volumes(&self) -> Result<()> {
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
            self.handle_mute_initial(source).await?;
        }
        Ok(())
    }

    async fn load_colours(&mut self) -> Result<()> {
        debug!("Loading Colour Map..");

        // Colours Schemes for Scribbles, Faders and Mute are handled in fader.rs

        // Configure the swear button..
        let target = TwoColourTargets::Swear;
        let swear_button = self.colour_scheme.get_two_colour_target(target);
        swear_button.colour1 = self.profile.swear.colours.active_colour;
        swear_button.colour2 = self.profile.swear.colours.inactive_colour;

        let command = BasicResultCommand::SetColour(self.colour_scheme);
        self.send_no_result(command).await
    }

    async fn apply_routing(&self) -> Result<()> {
        // Once we reach here, all routing changes should have been setup, so we apply routing
        // for all input channels.
        for channel in InputChannels::iter() {
            self.apply_routing_for_channel(channel).await?;
        }
        Ok(())
    }
}
