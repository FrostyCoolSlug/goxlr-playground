use std::path::PathBuf;

use enum_map::{enum_map, Enum, EnumMap};
use goxlr_types::FaderName;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use goxlr_shared::channels::OutputChannels;
use goxlr_shared::colours::Colour;
use goxlr_shared::faders::FaderSources;

mod types;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    // Channel First Approach!
    pub channels: EnumMap<FaderSources, FaderChannel>,
    pub pages: FaderPages,
}

#[derive(Serialize, Deserialize)]
pub struct FaderPages {
    pub current: usize,
    pub page_list: Vec<FaderPage>,
}

#[derive(Serialize, Deserialize)]
pub struct FaderPage {
    pub faders: EnumMap<FaderName, FaderSources>,
}

impl Default for FaderPage {
    fn default() -> Self {
        Self {
            faders: enum_map! {
                FaderName::A => FaderSources::Microphone,
                FaderName::B => FaderSources::Music,
                FaderName::C => FaderSources::Chat,
                FaderName::D => FaderSources::System
            },
        }
    }
}

/// This is a Channel that can be assigned to a fader. All configuration for the channel
/// including colours, mute states and behaviours are configured here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaderChannel {
    /// The current channel volume
    pub volume: u8,

    /// The current channel Mute State
    pub mute_state: MuteState,

    /// Defines what action is performed on Press and Hold
    pub mute_actions: EnumMap<MuteAction, MuteBehaviour>,

    /// The targets to be muted when 'MuteState::MutedToTarget' is active ('All' if emtpy)
    pub mute_targets: Vec<OutputChannels>,

    /// A struct detailing how a fader is displayed on the GoXLR
    pub display: FaderDisplay,
}

/// A struct that defines top to bottom how a fader is displayed on the Device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaderDisplay {
    /// Which display mode features to apply to the fader
    pub fader_display_mode: Vec<FaderDisplayMode>,

    /// The Colours assigned to the top and bottom of the fader
    pub fader_colours: FaderColourSet,

    /// The Colours assigned to the Mute button of the Fader
    pub mute_colours: ButtonColourSet,

    /// The setup of the screen at the top of a fader
    pub screen_display: Screen,
}

/// Represents the screen above the faders on the Full Sized GoXLR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screen {
    /// The background Colour of the Screen
    pub colour: Colour,

    /// Whether the Background and Foreground colours are inverted
    pub inverted: bool,

    /// A path to the icon to be displayed
    pub image: Option<PathBuf>,

    /// The text dislayed on the screen (central if no icon, at the bottom if icon)
    pub text: Option<String>,

    /// The Charater to display in the top left corner of the screen
    pub label: Option<char>,
}

/// This defines a Buttons colour configuration
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ButtonColourSet {
    /// The Currently Set 'Active' Colour
    pub active_colour: Colour,

    /// The Currently Set 'Inactive' Colour
    pub inactive_colour: Colour,

    /// How to represent a button when it's inactive.
    pub inactive_behaviour: InactiveButtonBehaviour,
}

/// Defines potential inactive button behaviours
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum InactiveButtonBehaviour {
    /// This Dimms the Active Colour.
    DimActive,

    /// This Dimms the inactive Colour.
    DimInactive,

    /// This brightly displays the inactive colour.
    InactiveColour,
}

/// Colour's related to the Fader Slider
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FaderColourSet {
    /// The colour displayed above the fader
    pub top_colour: Colour,

    /// The colour displayed below the fader
    pub bottom_colour: Colour,
}

/// How the colours on the fader are displayed
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum FaderDisplayMode {
    /// This will produce a meter, that matches the current audio volume
    Meter,

    /// This will display the colours as a Gradient from top_colour to bottom_colour
    Gradient,
}

/// These are the different methods of interacting with Mute Keys
#[derive(Debug, Copy, Clone, Enum, Serialize, Deserialize)]
pub enum MuteAction {
    Press,
    Hold,
}

/// This defines the two possible mute behaviours which can be assigned to keys
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MuteBehaviour {
    MuteToTarget,
    MuteToAll,
}

/// This represents the current state of a Channel
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MuteState {
    Unmuted,
    MutedToTarget,
    MutedToAll,
}

/// This needs to be improved..
impl Default for Profile {
    fn default() -> Self {
        // Lets build a profile, gotta start from the bottom up..
        let mute_action = enum_map! {
            MuteAction::Hold => MuteBehaviour::MuteToAll,
            MuteAction::Press => MuteBehaviour::MuteToTarget,
        };

        let display_mode = vec![FaderDisplayMode::Gradient];

        let green = Colour {
            red: 0,
            green: 255,
            blue: 0,
        };

        let colours = FaderColourSet {
            top_colour: Colour::default(),
            bottom_colour: green,
        };

        // We can set inactive colour to black, because we're Dimming the Active.
        let mute = ButtonColourSet {
            active_colour: green,
            inactive_colour: Default::default(),
            inactive_behaviour: InactiveButtonBehaviour::DimActive,
        };

        let display = Screen {
            colour: green,
            inverted: false,
            image: None,
            text: None,
            label: None,
        };

        let fader_display = FaderDisplay {
            fader_display_mode: display_mode,
            fader_colours: colours,
            mute_colours: mute,
            screen_display: display,
        };

        let channel = FaderChannel {
            volume: 0,
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action,
            mute_targets: Default::default(),
            display: fader_display,
        };

        // We're just going to clone this config out to all the channels, these would realistically
        // all be very different..
        let mut channels: EnumMap<FaderSources, FaderChannel> = enum_map! {
                FaderSources::Microphone => channel.clone(),
                FaderSources::Chat  => channel.clone(),
                FaderSources::Music => channel.clone(),
                FaderSources::Game => channel.clone(),
                FaderSources::Console => channel.clone(),
                FaderSources::LineIn => channel.clone(),
                FaderSources::System => channel.clone(),
                FaderSources::Sample => channel.clone(),
                FaderSources::Headphones => channel.clone(),
                FaderSources::LineOut => channel.clone(),
        };

        let base_colour: EnumMap<FaderSources, Colour> = enum_map! {
                FaderSources::Microphone => Colour {
                    red: 255,
                    green: 246,
                    blue: 84,
                },
                FaderSources::Chat  => Colour {
                    red: 36,
                    green: 255,
                    blue: 43,

                },
                FaderSources::Music => Colour {
                    red:42,
                    green: 255,
                    blue: 112,

                },
                FaderSources::Game => Colour {
                    red: 255,
                    green: 19,
                    blue: 142,

                },
                FaderSources::Console => Colour {
                    red: 86,
                    green: 14,
                    blue: 255,

                },
                FaderSources::LineIn => Colour {
                    red: 255,
                    green: 0,
                    blue: 0,

                },
                FaderSources::System => Colour {
                    red: 0,
                    green: 255,
                    blue: 0,

                },
                FaderSources::Sample => Colour {
                    red: 0,
                    green: 0,
                    blue: 255,

                },
                FaderSources::Headphones => Colour {
                    red: 255,
                    green: 36,
                    blue: 13,

                },
                FaderSources::LineOut => Colour {
                    red: 255,
                    green: 0,
                    blue: 255,

                },
        };

        // In the interests of testing, set the scribble to the name of the channel..
        for channel in FaderSources::iter() {
            channels[channel].display.screen_display.text = Some(format!("{:?}", channel));
            channels[channel].display.screen_display.colour = base_colour[channel];
            channels[channel].display.mute_colours.active_colour = base_colour[channel];
            channels[channel].display.screen_display.colour = base_colour[channel];
            channels[channel].display.fader_colours.bottom_colour = base_colour[channel];
        }

        let page = FaderPage::default();
        let page2 = FaderPage {
            faders: enum_map! {
                FaderName::A => FaderSources::System,
                FaderName::B => FaderSources::Game,
                FaderName::C => FaderSources::LineIn,
                FaderName::D => FaderSources::LineOut
            },
        };
        let page3 = FaderPage {
            faders: enum_map! {
                FaderName::A => FaderSources::Sample,
                FaderName::B => FaderSources::Chat,
                FaderName::C => FaderSources::Console,
                FaderName::D => FaderSources::Microphone
            },
        };

        let pages = FaderPages {
            current: 0,
            page_list: vec![page, page2, page3],
        };

        Profile { channels, pages }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
