use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use async_trait::async_trait;
use goxlr_profile::CoughBehaviour;
use log::debug;
use strum::IntoEnumIterator;

use goxlr_shared::buttons::Buttons;
use goxlr_shared::encoders::Encoders;
use goxlr_shared::faders::Fader;
use goxlr_shared::states::State;

use crate::device::goxlr::components::buttons::ButtonHandlers;
use crate::device::goxlr::components::mute_handler::MuteHandler;
use crate::device::goxlr::components::pages::FaderPages;
use crate::device::goxlr::components::profile::Profile;
use crate::device::goxlr::device::{ButtonState, GoXLR};

#[async_trait]
pub(crate) trait Interactions {
    async fn on_button_down(&mut self, button: Buttons) -> Result<()>;
    async fn on_button_up(&mut self, button: Buttons) -> Result<()>;
    async fn on_button_held(&mut self, button: Buttons) -> Result<()>;

    async fn on_volume_change(&mut self, fader: Fader, value: u8) -> Result<()>;
    async fn on_encoder_change(&mut self, encoder: Encoders, value: i8) -> Result<()>;

    async fn check_held(&mut self) -> Result<()>;
}

#[async_trait]
impl Interactions for GoXLR {
    async fn on_button_down(&mut self, button: Buttons) -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

        debug!("Button Down: {:?}", button);
        let mut skip_hold = false;
        let mut skip_release = false;

        match button {
            // Mute behaviours happen on button up, so we can use down to check paging here..
            Buttons::FaderA | Buttons::FaderB | Buttons::FaderC | Buttons::FaderD => {
                // Grab the button 'paired' with this one and check to see if it's pressed
                let pair = self.get_page_paired_button(button);
                if let Some(state) = self.button_down_states[pair] {
                    // Make sure the button hasn't been handled by something else
                    if !state.hold_handled {
                        // Check whether we're going forward or backwards through the pages
                        let previous_page = button == Buttons::FaderA || button == Buttons::FaderB;

                        // This internally handles the pressing of the button for release handling,
                        // so we're safe to return straight off of this.
                        return self.handle_page(pair, button, previous_page).await;
                    }
                }
            }
            Buttons::CoughButton => {
                if self.profile.cough.cough_behaviour == CoughBehaviour::Hold {
                    // We should apply the cough muting, and ignore hold behaviour.
                    skip_hold = true;
                }
            }

            Buttons::Swear => {
                // The swear button is super easy, we just turn it's light on..
                self.button_states.set_state(Buttons::Swear, State::Colour1);
                self.apply_button_states().await?;
            }
            _ => {}
        }

        // Register this button as down.
        self.button_down_states[button].replace(ButtonState {
            press_time: now,
            skip_hold,
            skip_release,
            hold_handled: false,
        });

        Ok(())
    }

    async fn on_button_up(&mut self, button: Buttons) -> Result<()> {
        debug!("Button Up: {:?}", button);
        if let Some(state) = self.button_down_states[button] {
            if state.skip_release {
                debug!("Skipping Button Up behaviour by request for {:?}", button);
                self.button_down_states[button].take();
                return Ok(());
            }
        }

        match button {
            Buttons::FaderA | Buttons::FaderB | Buttons::FaderC | Buttons::FaderD => {
                if !self.is_held_handled(button) {
                    let channel = self.get_channel_for_button(button);
                    self.handle_mute_press(channel).await?;
                }
            }
            Buttons::CoughButton => {
                // if self.profile.cough.cough_behaviour == CoughBehaviour::HOLD {
                //     // We need to unmute the cough button
                // } else {
                //     // We need to do something based on the current MuteState..
                // }
            }
            Buttons::Swear => {
                // Button released, revert to inactive state.
                let state = State::from(self.profile.swear.colours.inactive_behaviour);
                self.button_states.set_state(button, state);
                self.apply_button_states().await?;
            }
            _ => {
                // TODO: Remove this..
                self.button_states.set_state(button, State::DimmedColour1);
                self.apply_button_states().await?;
            }
        }

        // Regardless of outcome, we need to clear this from the button states..
        if self.button_down_states[button].is_some() {
            self.button_down_states[button].take();
        }

        Ok(())
    }

    async fn on_button_held(&mut self, button: Buttons) -> Result<()> {
        debug!("Button Held: {:?}", button);
        match button {
            Buttons::FaderA | Buttons::FaderB | Buttons::FaderC | Buttons::FaderD => {
                // Get the source assigned to this fader..
                let channel = self.get_channel_for_button(button);
                self.handle_mute_hold(channel).await?;
            }
            Buttons::CoughButton => {
                // We need to Trigger the 'Hold' behaviour..
            }
            _ => {
                // Nothing to do for this button
            }
        }

        Ok(())
    }

    async fn on_volume_change(&mut self, fader: Fader, value: u8) -> Result<()> {
        // Find the
        let current = self.profile.pages.current;
        let channel = self.profile.pages.page_list[current].faders[fader];

        debug!("Fader Moved: {:?} to {:?}", channel, value);
        self.profile.channels[channel].volume = value;

        Ok(())
    }

    async fn on_encoder_change(&mut self, encoder: Encoders, value: i8) -> Result<()> {
        debug!("Encoder {:?} changed to {}", encoder, value);
        match encoder {
            Encoders::Pitch => {}
            Encoders::Gender => {}
            Encoders::Reverb => {}
            Encoders::Echo => {}
        }

        Ok(())
    }

    async fn check_held(&mut self) -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        let hold_time = self.profile.configuration.button_hold_time;
        for button in Buttons::iter() {
            if let Some(mut state) = self.button_down_states[button] {
                if (now - state.press_time > hold_time.into())
                    && !state.hold_handled
                    && !state.skip_hold
                {
                    let _ = self.on_button_held(button).await;

                    // Set hold as handled, and store the change.
                    state.hold_handled = true;
                    self.button_down_states[button].replace(state);
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
trait InteractionsLocal {
    fn is_held_handled(&self, button: Buttons) -> bool;
    fn get_page_paired_button(&mut self, button: Buttons) -> Buttons;

    async fn handle_page(&mut self, one: Buttons, two: Buttons, prev: bool) -> Result<()>;
}

#[async_trait]
impl InteractionsLocal for GoXLR {
    fn is_held_handled(&self, button: Buttons) -> bool {
        if let Some(state) = self.button_down_states[button] {
            state.hold_handled
        } else {
            false
        }
    }

    /// Returns the paired button for fader paging
    fn get_page_paired_button(&mut self, button: Buttons) -> Buttons {
        match button {
            Buttons::FaderA => Buttons::FaderB,
            Buttons::FaderB => Buttons::FaderA,
            Buttons::FaderC => Buttons::FaderD,
            Buttons::FaderD => Buttons::FaderC,
            _ => {
                panic!("Invalid Button Passed!");
            }
        }
    }

    /// The 'first' button here is the button that was pressed first (which already exists in the
    /// button map), and the 'second' is the most recent button pressed.
    async fn handle_page(&mut self, one: Buttons, two: Buttons, prev: bool) -> Result<()> {
        let pages = self.profile.pages.page_list.len();
        let enabled = self.profile.configuration.change_page_with_buttons;

        if pages == 1 || !enabled {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

            // When there's only one page, don't activate the page behaviour. Simply put the button
            // into a 'down' state and return.
            self.button_down_states[two].replace(ButtonState {
                press_time: now,
                skip_hold: false,
                skip_release: false,
                hold_handled: false,
            });

            return Ok(());
        }

        if let Some(mut state) = self.button_down_states[one] {
            if prev {
                self.prev_page().await?;
            } else {
                self.next_page().await?;
            }

            // Skip future behaviours for this button..
            state.skip_hold = true;
            state.skip_release = true;
            self.button_down_states[one].replace(state);

            // We also need to skip behaviours for ourself, so register that now..
            self.button_down_states[two].replace(ButtonState {
                press_time: 0,
                skip_hold: true,
                skip_release: true,
                hold_handled: false,
            });
        } else {
            // This should never happen, code calling into here have confirmed that the first
            // button already exists
            panic!("Button State Mismatch for paging!");
        }
        return Ok(());
    }
}
