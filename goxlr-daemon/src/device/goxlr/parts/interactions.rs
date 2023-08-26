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
use crate::device::goxlr::parts::load_profile::LoadProfile;

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
        debug!("Button Down: {:?}", button);
        match button {
            _ => {
                // TODO: Remove this..
                // For initial testing, we'll just light up the button, make it flash if it's held
                // then reset it when it's released.
                self.button_states.set_state(button, State::Colour1);
                self.apply_button_states().await?;

                // By default, we simply store the current time that this was held, and handle
                // buttons that are held later.
                let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
                self.button_down_states[button].replace(ButtonState {
                    press_time: now,
                    hold_handled: false,
                });
            }
        }

        Ok(())
    }

    async fn on_button_up(&mut self, button: Buttons) -> Result<()> {
        debug!("Button Up: {:?}", button);

        match button {
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
                if (now - state.press_time > hold_time.into()) && !state.hold_handled {
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
