use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

use enum_map::EnumMap;
use enumset::EnumSet;
use goxlr_profile::{FaderPage, Profile};
use goxlr_scribbles::get_scribble;
use goxlr_types::FaderName;
use goxlr_usb::buttonstate::{ButtonStates, Buttons};
use goxlr_usb::colours;
use goxlr_usb::colours::TwoColourTargets::{
    Fader1Mute, Fader2Mute, Fader3Mute, Fader4Mute, Scribble1, Scribble2, Scribble3, Scribble4,
};
use goxlr_usb::colours::{ColourScheme, FaderTarget};
use goxlr_usb::device::base::FullGoXLRDevice;
use strum::IntoEnumIterator;

use tokio::select;
use tokio::sync::mpsc::Receiver;

// This is an incredibly cheap and cheerful cut down version of the device in the util..
pub struct Device {
    goxlr: Box<dyn FullGoXLRDevice>,
    current_pressed: EnumSet<Buttons>,
    button_states: EnumMap<Buttons, ButtonState>,
    colour_scheme: ColourScheme,
    event_receiver: Receiver<String>,
    profile: Profile,
}

impl Device {
    pub async fn new(
        goxlr: Box<dyn FullGoXLRDevice>,
        event_receiver: Receiver<String>,
        profile: Profile,
    ) -> Result<Self> {
        // EZPZ.
        Ok(Self {
            goxlr,
            current_pressed: EnumSet::default(),
            button_states: EnumMap::default(),
            colour_scheme: ColourScheme::default(),
            event_receiver,
            profile,
        })
    }

    pub async fn run_handler(&mut self) {
        loop {
            select! {
                // This only works with 1 device right now..
                Some(_) = self.event_receiver.recv() => {
                    let _ = self.monitor_inputs().await;
                }
            }
        }
    }

    pub async fn monitor_inputs(&mut self) -> Result<()> {
        // We only care about button states..
        let state = self.goxlr.get_button_states()?;

        // Get a list of newly pressed buttons..
        let pressed_buttons = state.pressed.difference(self.current_pressed);
        for button in pressed_buttons {
            // New Button pressed..
            self.button_states[button] = ButtonState {
                press_time: get_epoch_ms(),
                handled: false,
            };

            let _ = self.on_button_down(button).await;
        }

        // And flip it for button release..
        let released_buttons = self.current_pressed.difference(state.pressed);
        for button in released_buttons {
            let _ = self.on_button_up(button).await;
            self.button_states[button] = ButtonState::default();
        }

        self.current_pressed = state.pressed;
        Ok(())
    }

    pub async fn on_button_down(&mut self, button: Buttons) {
        match button {
            Buttons::Fader1Mute => {
                if self.current_pressed.contains(Buttons::Fader2Mute) {
                    println!("Fader Page Back");
                    self.page_back().await;
                }
            }
            Buttons::Fader2Mute => {
                if self.current_pressed.contains(Buttons::Fader1Mute) {
                    println!("Fader Page Back");
                    self.page_back().await;
                }
            }
            Buttons::Fader3Mute => {
                if self.current_pressed.contains(Buttons::Fader4Mute) {
                    println!("Fader Page Forward");
                    self.page_forward().await;
                }
            }
            Buttons::Fader4Mute => {
                if self.current_pressed.contains(Buttons::Fader3Mute) {
                    println!("Fader Page Forward");
                    self.page_forward().await;
                }
            }
            _ => {}
        }
    }

    async fn page_back(&mut self) {
        let total_pages = self.profile.pages.page_list.len();
        if total_pages == 0 || total_pages == 1 {
            // Can't page anywhere..
            return;
        }
        let new_page = if self.profile.pages.current == 0 {
            total_pages - 1
        } else {
            self.profile.pages.current - 1
        };

        self.profile.pages.current = new_page;
        self.load_fader_settings().await;
        self.apply_colours().await;
    }

    async fn page_forward(&mut self) {
        let total_pages = self.profile.pages.page_list.len();
        if total_pages == 0 || total_pages == 1 {
            // Can't page anywhere..
            return;
        }
        let new_page = if self.profile.pages.current == (total_pages - 1) {
            0
        } else {
            self.profile.pages.current + 1
        };

        self.profile.pages.current = new_page;
        self.load_fader_settings().await;
        self.apply_colours().await;
        self.apply_button_states().await;
    }

    pub async fn on_button_up(&self, _button: Buttons) {
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
        let default = FaderPage::default();
        let page = if self.profile.pages.page_list.is_empty() {
            &default
        } else {
            &self.profile.pages.page_list[self.profile.pages.current]
        };

        for fader in FaderTarget::iter() {
            let source = match fader {
                FaderTarget::FaderA => page.fader_a,
                FaderTarget::FaderB => page.fader_b,
                FaderTarget::FaderC => page.fader_c,
                FaderTarget::FaderD => page.fader_d,
            };

            // Grab the channel for this fader..
            let channel = &self.profile.channels[source];

            // Set the Fader Top / Bottom Colours
            let mut target = self.colour_scheme.get_fader_target(fader);
            target.colours.colour1 = map_colour_to_usb(channel.display.fader_colours.top_colour);
            target.colours.colour2 = map_colour_to_usb(channel.display.fader_colours.bottom_colour);

            // Set the Fader Screen Colour..
            let scribble = match fader {
                FaderTarget::FaderA => self.colour_scheme.get_two_colour_target(Scribble1),
                FaderTarget::FaderB => self.colour_scheme.get_two_colour_target(Scribble2),
                FaderTarget::FaderC => self.colour_scheme.get_two_colour_target(Scribble3),
                FaderTarget::FaderD => self.colour_scheme.get_two_colour_target(Scribble4),
            };
            scribble.colour1 = map_colour_to_usb(channel.display.screen_display.colour);

            // Set the Mute Button colours..
            let mute = match fader {
                FaderTarget::FaderA => self.colour_scheme.get_two_colour_target(Fader1Mute),
                FaderTarget::FaderB => self.colour_scheme.get_two_colour_target(Fader2Mute),
                FaderTarget::FaderC => self.colour_scheme.get_two_colour_target(Fader3Mute),
                FaderTarget::FaderD => self.colour_scheme.get_two_colour_target(Fader4Mute),
            };
            mute.colour1 = map_colour_to_usb(channel.display.mute_colours.active_colour);
            mute.colour2 = map_colour_to_usb(channel.display.mute_colours.inactive_colour);

            let fader_name = match fader {
                FaderTarget::FaderA => FaderName::A,
                FaderTarget::FaderB => FaderName::B,
                FaderTarget::FaderC => FaderName::C,
                FaderTarget::FaderD => FaderName::D,
            };

            let screen = channel.display.screen_display.clone();

            // Now to render the scribbles :D
            let scribble = get_scribble(screen.image, screen.text, None, screen.inverted);
            let _ = self.goxlr.set_fader_scribble(fader_name, scribble);

            // Now assign the channel to the fader..
            let _ = self
                .goxlr
                .set_fader(fader_name, source.get_source().channel_map);
        }
    }

    pub async fn apply_colours(&mut self) {
        let mut slice: [u8; 520] = [0; 520];

        slice.copy_from_slice(&self.colour_scheme.build_colour_map(false)[0..520]);
        let _ = self.goxlr.set_button_colours_1_3_40(slice);
    }

    pub async fn apply_button_states(&mut self) {
        let state = [ButtonStates::DimmedColour1; 24];
        let _ = self.goxlr.set_button_states(state);
    }
}

fn map_colour_to_usb(profile: goxlr_profile::Colour) -> colours::Colour {
    colours::Colour {
        red: profile.red as u32,
        green: profile.green as u32,
        blue: profile.blue as u32,
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
