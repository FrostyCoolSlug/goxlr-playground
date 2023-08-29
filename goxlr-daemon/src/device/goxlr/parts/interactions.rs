use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use strum::IntoEnumIterator;

use goxlr_shared::buttons::Buttons;
use goxlr_shared::encoders::Encoders;
use goxlr_shared::faders::Fader;
use goxlr_shared::states::State;

use crate::device::goxlr::device::{ButtonState, GoXLR};
use crate::device::goxlr::parts::buttons::ButtonHandlers;
use crate::device::goxlr::parts::mute_handler::MuteHandler;
use crate::device::goxlr::parts::pages::FaderPages;
use crate::device::goxlr::parts::profile::Profile;

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
        match button {
            Buttons::FaderA => {
                // This exists on button down for all fader buttons, it basically determines
                // whether we should page for a button combination.
                if let Some(state) = self.button_down_states[Buttons::FaderB] {
                    if !state.hold_handled {
                        // This internally handles the pressing of the button for release handling,
                        // so we're safe to return off of this.
                        return self.handle_page(Buttons::FaderB, button, true).await;
                    }
                }
            }
            Buttons::FaderB => {
                if let Some(state) = self.button_down_states[Buttons::FaderA] {
                    if !state.hold_handled {
                        return self.handle_page(Buttons::FaderA, button, true).await;
                    }
                }
            }
            Buttons::FaderC => {
                if let Some(state) = self.button_down_states[Buttons::FaderD] {
                    if !state.hold_handled {
                        return self.handle_page(Buttons::FaderD, button, false).await;
                    }
                }
            }
            Buttons::FaderD => {
                if let Some(state) = self.button_down_states[Buttons::FaderC] {
                    if !state.hold_handled {
                        return self.handle_page(Buttons::FaderC, button, false).await;
                    }
                }
            }

            Buttons::Swear => {}
            _ => {}
        }

        // Register this button as down.

        self.button_down_states[button].replace(ButtonState {
            press_time: now,
            skip_hold: false,
            skip_release: false,
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
            _ => {
                self.button_states.set_state(button, State::Blinking);
                self.apply_button_states().await?;
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

    /// The 'first' button here is the button that was pressed first (which already exists in the
    /// button map), and the 'second' is the most recent button pressed.
    async fn handle_page(&mut self, one: Buttons, two: Buttons, prev: bool) -> Result<()> {
        if self.profile.pages.page_list.len() == 1 {
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
