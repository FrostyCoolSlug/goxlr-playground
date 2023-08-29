/* This file primarily handles interactions with the GoXLR, we track the basic state of buttons,
   (pressed / unpressed) and (old value / new value), and send events upstream to handle them.

   We're not responsible for any 'Hold' behaviours here, as they aren't technically GoXLR related
   functionality, instead are handled upstream.

   Nor do we need to care about what fader is assigned to what, nor it's volume. All we care about
   here is whether something has physically changed on the GoXLR.
*/

use enum_map::EnumMap;
use enumset::EnumSet;
use strum::IntoEnumIterator;
use tokio::sync::mpsc;

use goxlr_shared::interaction::{
    ButtonStates, InteractiveButtons, InteractiveEncoders, InteractiveFaders,
};

use crate::events::interaction::InteractionEvent;
use crate::types::buttons::{CurrentButtonStates, PhysicalButton};

#[derive(Debug)]
pub(crate) struct StateTracker {
    sender: mpsc::Sender<InteractionEvent>,

    first_run: bool,
    button_states: EnumMap<InteractiveButtons, ButtonStates>,
    volume_map: EnumMap<InteractiveFaders, u8>,
    encoder_map: EnumMap<InteractiveEncoders, i8>,
}

impl StateTracker {
    pub fn new(sender: mpsc::Sender<InteractionEvent>) -> Self {
        Self {
            sender,

            first_run: true,
            button_states: EnumMap::default(),
            volume_map: EnumMap::default(),
            encoder_map: EnumMap::default(),
        }
    }

    /// Called when a GetStatus response has completed, we check for any changes to the state
    /// and trigger events events for any confirmed changes.
    pub async fn update_states(&mut self, states: CurrentButtonStates) {
        self.update_volumes(states.volumes).await;
        self.update_encoders(states.encoders).await;
        self.update_buttons(states.pressed).await;

        self.first_run = false;
    }

    async fn update_volumes(&mut self, volumes: [u8; 4]) {
        for fader in InteractiveFaders::iter() {
            let volume = volumes[fader as usize];
            if self.volume_map[fader] != volume || self.first_run {
                self.volume_map[fader] = volumes[fader as usize];
                let _ = self
                    .sender
                    .send(InteractionEvent::VolumeChange(fader, volume))
                    .await;
            }
        }
    }

    async fn update_encoders(&mut self, encoders: [i8; 4]) {
        for encoder in InteractiveEncoders::iter() {
            let value = encoders[encoder as usize];
            if self.encoder_map[encoder] != value || self.first_run {
                self.encoder_map[encoder] = value;
                let _ = self
                    .sender
                    .send(InteractionEvent::EncoderChange(encoder, value))
                    .await;
            }
        }
    }

    async fn update_buttons(&mut self, buttons: EnumSet<PhysicalButton>) {
        for button in InteractiveButtons::iter() {
            let current_state = self.button_states[button];
            let status_button = PhysicalButton::from(button);

            if buttons.contains(status_button) && current_state == ButtonStates::NotPressed {
                let _ = self.sender.send(InteractionEvent::ButtonDown(button)).await;
                self.button_states[button] = ButtonStates::Pressed;
            }
            if !buttons.contains(status_button) && current_state == ButtonStates::Pressed {
                let _ = self.sender.send(InteractionEvent::ButtonUp(button)).await;
                self.button_states[button] = ButtonStates::NotPressed;
            }
        }
    }
}
