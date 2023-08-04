use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use log::debug;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::{task, time};

use goxlr_shared::interaction::InteractiveButtons;
use goxlr_usb::platform::unix::device_handler::spawn_device_handler;
use goxlr_usb::pnp_base::DeviceEvents;
use goxlr_usb::{ChangeEvent, GoXLRDevice};

#[derive(Clone)]
pub struct Device {
    inner: Arc<Mutex<DeviceInner>>,
}

impl Device {
    pub async fn new(device: GoXLRDevice, device_sender: Sender<DeviceEvents>) -> Result<Self> {
        // Create the event receiver..
        let (event_sender, event_receiver) = mpsc::channel(32);

        let inner = DeviceInner::new(device, event_sender, device_sender).await?;
        let stop = inner.should_stop.clone();
        let wrapped = Arc::new(Mutex::new(inner));

        task::spawn(spawn_event_handler(wrapped.clone(), event_receiver, stop));

        Ok(Self {
            inner: wrapped.clone(),
        })
    }

    pub async fn stop(&self) {
        self.inner
            .lock()
            .await
            .should_stop
            .store(true, Ordering::Relaxed);
    }
}

pub struct DeviceInner {
    /// This is simply the USB identification of a device, to save a lot of hassle, it's the primary
    /// lookup method for attack / detach events.
    device: GoXLRDevice,

    /// Everything that runs as it's own task should include a clone of this, when a GoXLR is
    /// disconnected, this is set to true, and everything below should be stopped.
    should_stop: Arc<AtomicBool>,

    /// A sender which allows us to message the primary worker in the event something goes
    /// tragically wrong, this will force a destruction of the existing state, and attempt to
    /// create a new clean one.
    device_sender: Sender<DeviceEvents>,
}

impl DeviceInner {
    pub async fn new(
        device: GoXLRDevice,
        event_sender: Sender<ChangeEvent>,
        device_sender: Sender<DeviceEvents>,
    ) -> Result<Self> {
        let device_inner = Self {
            device: device.clone(),
            should_stop: Arc::new(AtomicBool::new(false)),
            device_sender: device_sender.clone(),
            //event_sender,
        };

        let (ready_send, ready_recv) = oneshot::channel();
        task::spawn(spawn_device_handler(
            device,
            ready_send,
            event_sender,
            device_sender,
        ));

        // Wait for the handler to give us an 'OK' before proceeding..
        ready_recv.await??;

        Ok(device_inner)
    }

    pub async fn handle_change_event(&mut self, event: ChangeEvent) {
        match event {
            ChangeEvent::VolumeChange(fader, volume) => {
                debug!("Volume Changed for Fader {:?} to {}", fader, volume);
            }
            ChangeEvent::ButtonDown(button) => {
                debug!("Button {:?} Pressed", button);
            }
            ChangeEvent::ButtonUp(button) => {
                debug!("Button {:?} Released", button);
            }
            ChangeEvent::EncoderChange(encoder, value) => {
                debug!("Encoder {:?} changed to {}", encoder, value);
            }
        }
    }

    pub async fn monitor_inputs(&mut self) -> Result<()> {
        // We only care about button states..
        //let state = self.goxlr.get_button_states()?;

        // Get a list of newly pressed buttons..
        // let pressed_buttons = state.pressed.difference(self.current_pressed);
        // for button in pressed_buttons {
        //     // New Button pressed..
        //     self.button_states[button] = ButtonState {
        //         press_time: get_epoch_ms(),
        //         handled: false,
        //     };
        //
        //     let _ = self.on_button_down(button).await;
        // }
        // And flip it for button release..
        // let released_buttons = self.current_pressed.difference(state.pressed);
        // for button in released_buttons {
        //     let _ = self.on_button_up(button).await;
        //     self.button_states[button] = ButtonState::default();
        // }
        //
        // self.current_pressed = state.pressed;
        Ok(())
    }

    pub async fn on_button_down(&mut self, button: InteractiveButtons) {
        // match button {
        //     Buttons::Fader1Mute => {
        //         if self.current_pressed.contains(Buttons::Fader2Mute) {
        //             println!("Fader Page Back");
        //             self.page_back().await;
        //         }
        //     }
        //     Buttons::Fader2Mute => {
        //         if self.current_pressed.contains(Buttons::Fader1Mute) {
        //             println!("Fader Page Back");
        //             self.page_back().await;
        //         }
        //     }
        //     Buttons::Fader3Mute => {
        //         if self.current_pressed.contains(Buttons::Fader4Mute) {
        //             println!("Fader Page Forward");
        //             self.page_forward().await;
        //         }
        //     }
        //     Buttons::Fader4Mute => {
        //         if self.current_pressed.contains(Buttons::Fader3Mute) {
        //             println!("Fader Page Forward");
        //             self.page_forward().await;
        //         }
        //     }
        //     _ => {}
        // }
    }

    async fn page_back(&mut self) {
        // let total_pages = self.profile.pages.page_list.len();
        // if total_pages == 0 || total_pages == 1 {
        //     // Can't page anywhere..
        //     return;
        // }
        // let new_page = if self.profile.pages.current == 0 {
        //     total_pages - 1
        // } else {
        //     self.profile.pages.current - 1
        // };
        //
        // self.profile.pages.current = new_page;
        // self.load_fader_settings().await;
        // self.apply_colours().await;
    }

    async fn page_forward(&mut self) {
        // let total_pages = self.profile.pages.page_list.len();
        // if total_pages == 0 || total_pages == 1 {
        //     // Can't page anywhere..
        //     return;
        // }
        // let new_page = if self.profile.pages.current == (total_pages - 1) {
        //     0
        // } else {
        //     self.profile.pages.current + 1
        // };
        //
        // self.profile.pages.current = new_page;
        // self.load_fader_settings().await;
        // self.apply_colours().await;
        // self.apply_button_states().await;
    }

    pub async fn on_button_up(&self, _button: InteractiveButtons) {
        // We do nothing right now.
    }

    pub async fn load_profile(&mut self) {
        // Load the fader settings..
        self.load_fader_settings().await;

        // Apply the new colours
        self.apply_colours().await;

        // Set the button States..
        self.apply_button_states().await;
    }

    pub async fn load_fader_settings(&mut self) {
        // let default = FaderPage::default();
        // let page = if self.profile.pages.page_list.is_empty() {
        //     &default
        // } else {
        //     &self.profile.pages.page_list[self.profile.pages.current]
        // };
        //
        // for fader in Fader::iter() {
        //     let source = page.faders[fader];
        //
        //     // Grab the channel for this fader..
        //     let channel = &self.profile.channels[source];
        //
        //     // Set the Fader Top / Bottom Colours
        //     let mut target = self.colour_scheme.get_fader_target(fader);
        //     target.colour1 = channel.display.fader_colours.top_colour;
        //     target.colour2 = channel.display.fader_colours.bottom_colour;
        //
        //     // Set the Fader Screen Colour..
        //     let scribble = match fader {
        //         Fader::A => self.colour_scheme.get_two_colour_target(Scribble1),
        //         Fader::B => self.colour_scheme.get_two_colour_target(Scribble2),
        //         Fader::C => self.colour_scheme.get_two_colour_target(Scribble3),
        //         Fader::D => self.colour_scheme.get_two_colour_target(Scribble4),
        //     };
        //     scribble.colour1 = channel.display.screen_display.colour;
        //
        //     // Set the Mute Button colours..
        //     let mute = match fader {
        //         Fader::A => self.colour_scheme.get_two_colour_target(Fader1Mute),
        //         Fader::B => self.colour_scheme.get_two_colour_target(Fader2Mute),
        //         Fader::C => self.colour_scheme.get_two_colour_target(Fader3Mute),
        //         Fader::D => self.colour_scheme.get_two_colour_target(Fader4Mute),
        //     };
        //     mute.colour1 = channel.display.mute_colours.active_colour;
        //     mute.colour2 = channel.display.mute_colours.inactive_colour;
        //
        //     let screen = channel.display.screen_display.clone();

        // Now to render the scribbles :D
        //let scribble = get_scribble(screen.image, screen.text, None, screen.inverted);
        // let _ = self.goxlr.set_fader_scribble(fader_name, scribble);

        // Now assign the channel to the fader..
        // let _ = self
        //     .goxlr
        //     .set_fader(fader_name, source.get_source().channel_map);
        //}
    }

    pub async fn apply_colours(&mut self) {
        // let mut slice: [u8; 520] = [0; 520];
        //
        // slice.copy_from_slice(&self.colour_scheme.build_colour_map(false)[0..520]);
        // let _ = self.goxlr.set_button_colours_1_3_40(slice);
    }

    pub async fn apply_button_states(&mut self) {
        // let state = [ButtonStates::DimmedColour1; 24];
        // let _ = self.goxlr.set_button_states(state);
    }

    pub async fn stop_handler(&self) {
        debug!("Attempting to Stop..");
        self.should_stop.store(true, Ordering::Relaxed);
    }
}

#[derive(Default, Copy, Clone)]
struct ButtonState {
    press_time: u128,
    handled: bool,
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

async fn spawn_event_handler(
    device: Arc<Mutex<DeviceInner>>,
    mut event_receiver: mpsc::Receiver<ChangeEvent>,
    stop_signal: Arc<AtomicBool>,
) {
    let mut ticker = time::interval(Duration::from_millis(20));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if stop_signal.load(Ordering::Relaxed) {
                    debug!("Asked to Stop!");
                    break;
                }
            }
            Some(event) = event_receiver.recv() => {
                // Simply pass this event to our inner handler.
                device.lock().await.handle_change_event(event).await;
            }
        }
    }
}
