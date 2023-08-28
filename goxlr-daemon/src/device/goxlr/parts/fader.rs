use anyhow::Result;
use async_trait::async_trait;
use goxlr_scribbles::get_scribble;
use goxlr_shared::buttons::Buttons;
use log::debug;

use goxlr_shared::device::{DeviceType, GoXLRFeature};
use goxlr_shared::faders::{Fader, FaderSources};
use goxlr_shared::scribbles::Scribble;
use goxlr_usb_messaging::events::commands::{BasicResultCommand, ChannelSource};

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::parts::mute_handler::MuteHandler;

const SUBMIX_MITIGATION: &[FaderSources] = &[FaderSources::Headphones, FaderSources::LineOut];

/// This trait is responsible for the management of faders, everything from the top of the
/// scribble display, to the bottom of the mute button. Any changes which are to occur to them
/// should make their way through here.
#[async_trait]
pub(crate) trait DeviceFader {
    async fn assign_fader(&mut self, fader: Fader, source: FaderSources) -> Result<()>;
}

#[async_trait]
impl DeviceFader for GoXLR {
    /// Ok, there are multiple steps when assigning a fader, we need to assign it, update the
    /// colours and the fader style, the scribble display, and the mute button state.
    ///
    /// Some settings may not need to be immediately applied (such as colours and mute state) as
    /// it makes more sense to apply them all at once if assigning multiple faders.
    async fn assign_fader(&mut self, fader: Fader, source: FaderSources) -> Result<()> {
        // Get the details for the source..
        let details = self.profile.channels[source].clone();

        debug!("Assigning {:?} to {:?}", source, fader);
        // Assign the fader..
        let command_source = ChannelSource::FromFaderSource(source);
        let command = BasicResultCommand::AssignFader(fader, command_source);
        self.send_no_result(command).await?;

        debug!("Performing Submix mitigation Check");
        // Submix mitigation code, assigning output channels to faders can cause their volume to
        // spike to 100%, the code here immediately resets their volume back to where it should be.
        if SUBMIX_MITIGATION.contains(&source) {
            if let Some(device) = &self.device {
                if device.features.contains(&GoXLRFeature::Submix) {
                    let volume = details.volume;
                    debug!("Mitigating, Setting Volume of {:?} to {:?}", source, volume);
                    let command = BasicResultCommand::SetVolume(command_source, volume);
                    self.send_no_result(command).await?;
                }
            }
        }

        // Set the Faders Colour Style..
        let display_mode = details.display.fader_display_mode;
        debug!("Setting Fader {:?} display to {:?}", fader, display_mode);
        let command = BasicResultCommand::SetFaderStyle(fader, display_mode);
        self.send_no_result(command).await?;

        debug!("Colours: Screen, Fader and Mute Button for {:?}", fader);
        // Start setting up colours..
        let fader_colours = self.colour_scheme.get_fader_target(fader);
        fader_colours.colour1 = details.display.fader_colours.top_colour;
        fader_colours.colour2 = details.display.fader_colours.bottom_colour;

        // While in the colour structure, scribbles have two colours, there's only one actually used.
        let scribble = Scribble::from(fader).into();
        let scribble_colour = self.colour_scheme.get_two_colour_target(scribble);
        scribble_colour.colour1 = details.display.screen_display.colour;

        // Set the colours for the mute button..
        let mute_button = Buttons::from_fader(fader);
        let mute_colours = self.colour_scheme.get_two_colour_target(mute_button.into());
        mute_colours.colour1 = details.display.mute_colours.active_colour;
        mute_colours.colour2 = details.display.mute_colours.inactive_colour;

        debug!("{:#?}", mute_colours);

        if let Some(device) = &self.device {
            if device.device_type != DeviceType::Mini {
                let text = format!("{:?}", source);
                debug!("Setting Screen Text to {:?}", text);
                let scribble = get_scribble(None, Some(text), None, false);
                let command = BasicResultCommand::SetScribble(fader, scribble);
                self.send_no_result(command).await?;
            }
        }

        // Get the button mute state for this channel..
        debug!("Loading Mute button state for {:?}", source);
        let state = self.get_mute_button_state(source);
        self.button_states.set_state(mute_button, state);

        Ok(())
    }
}
