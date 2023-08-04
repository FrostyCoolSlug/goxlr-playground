// This file primarily handles interactions with the GoXLR, we track the basic state of buttons,
// (pressed / unpressed) and (old value / new value), and send events upstream to handle them.

// We're not responsible for any 'Hold' behaviours here, as they aren't technically GoXLR related
// functionality, instead are handled upstream.

// Nor do we need to care about what fader is assigned to what, nor it's volume. All we care about
// here is whether something has physically changed on the GoXLR.

use crate::button_state::{CurrentButtonStates, StatusButton};
use crate::ChangeEvent;
use enum_map::EnumMap;
use enumset::EnumSet;
use goxlr_shared::interaction::{
    ButtonState, InteractiveButtons, InteractiveEncoders, InteractiveFaders,
};
use strum::IntoEnumIterator;
use tokio::sync::mpsc;

#[derive(Debug)]
pub(crate) struct GoXLRStateTracker {
    button_states: EnumMap<InteractiveButtons, ButtonState>,
    volume_map: EnumMap<InteractiveFaders, u8>,
    encoder_map: EnumMap<InteractiveEncoders, i8>,

    // Under Linux, the receiver will be the timed poller, and under Windows the receiver will
    // handle events received from TUSB. This class need to be generic, and simply handle the IO
    // from other locations.
    sender: mpsc::Sender<ChangeEvent>,
}

// This needs to be platform specific, on Windows we use the TUSB event handler to trigger events,
// where as under Linux, we use RUSB polling for events. We need to cleanly support both.

impl GoXLRStateTracker {
    pub fn new(sender: mpsc::Sender<ChangeEvent>) -> Self {
        Self {
            sender,

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
    }

    async fn update_volumes(&mut self, volumes: [u8; 4]) {
        for fader in InteractiveFaders::iter() {
            let volume = volumes[fader as usize];
            if self.volume_map[fader] != volume {
                self.volume_map[fader] = volumes[fader as usize];
                let _ = self
                    .sender
                    .send(ChangeEvent::VolumeChange(fader, volume))
                    .await;
            }
        }
    }

    async fn update_encoders(&mut self, encoders: [i8; 4]) {
        for encoder in InteractiveEncoders::iter() {
            let value = encoders[encoder as usize];
            if self.encoder_map[encoder] != value {
                self.encoder_map[encoder] = value;
                let _ = self
                    .sender
                    .send(ChangeEvent::EncoderChange(encoder, value))
                    .await;
            }
        }
    }

    async fn update_buttons(&mut self, buttons: EnumSet<StatusButton>) {
        for button in InteractiveButtons::iter() {
            let current_state = self.button_states[button];
            let status_button = StatusButton::from(button);

            if buttons.contains(status_button) && current_state == ButtonState::NotPressed {
                let _ = self.sender.send(ChangeEvent::ButtonDown(button)).await;
                self.button_states[button] = ButtonState::Pressed;
            }
            if !buttons.contains(status_button) && current_state == ButtonState::Pressed {
                let _ = self.sender.send(ChangeEvent::ButtonUp(button)).await;
                self.button_states[button] = ButtonState::NotPressed;
            }
        }
    }
}
