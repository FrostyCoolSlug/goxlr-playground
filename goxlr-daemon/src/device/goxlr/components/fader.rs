use anyhow::Result;
use goxlr_scribbles::get_scribble;
use log::debug;

use goxlr_shared::buttons::Buttons;
use goxlr_shared::channels::MuteState;
use goxlr_shared::device::{DeviceType, GoXLRFeature};
use goxlr_shared::faders::{Fader, FaderSources};
use goxlr_shared::scribbles::Scribble;
use goxlr_usb::events::commands::{BasicResultCommand, ChannelSource};

use crate::device::goxlr::components::buttons::ButtonHandlers;
use crate::device::goxlr::components::mute_handler::MuteHandler;
use crate::device::goxlr::components::profile::Profile;
use crate::device::goxlr::device::GoXLR;

const SUBMIX_MITIGATION: &[FaderSources] = &[
    FaderSources::Headphones,
    FaderSources::LineOut,
    FaderSources::MicrophoneMonitor,
];

/// This trait is responsible for the management of faders, everything from the top of the
/// scribble display, to the bottom of the mute button. Any changes which are to occur to them
/// should make their way through here.
pub(crate) trait DeviceFader {
    async fn assign_fader(&mut self, fader: Fader, source: FaderSources) -> Result<()>;
    async fn update_mute_state(&mut self, source: FaderSources, state: MuteState) -> Result<()>;
}

impl DeviceFader for GoXLR {
    /// Ok, there are multiple steps when assigning a fader, we need to assign it, update the
    /// colours and the fader style, the scribble display, and the mute button state.
    ///
    /// Some settings may not need to be immediately applied (such as colours and mute state) as
    /// it makes more sense to apply them all at once if assigning multiple faders.
    async fn assign_fader(&mut self, fader: Fader, source: FaderSources) -> Result<()> {
        // Get the details for the source..
        let details = self.profile.channels[source].clone();
        let command_source = ChannelSource::FromFaderSource(source);

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

        let command = BasicResultCommand::AssignFader(fader, command_source);
        self.send_no_result(command).await?;

        // Update the Cache after assignment
        self.fader_state[fader].replace(source);

        debug!("Performing Submix mitigation Check");
        // Submix mitigation code, assigning output channels to faders can cause their volume to
        // spike to 100%, the code here immediately resets their volume back to where it should be.
        if SUBMIX_MITIGATION.contains(&source) {
            if let Some(device) = &self.device {
                if device.features.contains(&GoXLRFeature::Submix) {
                    let volume = details.volume.mix_a;
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

    /// Called when the mute state gets updated (probably from mute_handler.rs) to update the
    /// button state.
    async fn update_mute_state(&mut self, source: FaderSources, state: MuteState) -> Result<()> {
        self.profile.channels[source].mute_state = state;
        if let Some(button) = self.get_button_for_channel(source) {
            let state = self.get_mute_button_state(source);
            self.button_states.set_state(button, state);
            self.apply_button_states().await?;
        }
        Ok(())
    }
}
