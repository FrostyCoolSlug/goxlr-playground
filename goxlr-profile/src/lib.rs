use std::path::PathBuf;

use enum_map::{enum_map, Enum, EnumMap};
use serde::{Deserialize, Serialize};

use goxlr_shared::buttons::InactiveButtonBehaviour;
use goxlr_shared::channels::{InputChannels, MuteState, OutputChannels};
use goxlr_shared::colours::{Colour, FaderColour, FaderDisplayMode, TwoColour};
use goxlr_shared::faders::{Fader, FaderSources};

mod default;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// All the Assignable Channels, and their settings..
    pub channels: EnumMap<FaderSources, FaderChannel>,

    /// Fader Paging Configuration
    pub pages: FaderPages,

    /// Configuration for the Swear Button
    pub swear: SwearSettings,

    pub cough: CoughSettings,

    /// The Routing Configuration
    ///
    /// Note, we don't use RoutingOutput here, as the HardTune setting is entirely transient thus
    /// shouldn't be stored in the profile. You can use .into() to get it's RoutingOutput equivalent.
    pub routing: EnumMap<InputChannels, EnumMap<OutputChannels, bool>>,

    /// The General 'Configuration' of the device, this holds various settings such as (hold time)
    /// and any other settings that don't really fit anywhere else.
    pub configuration: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaderPages {
    pub current: usize,
    pub page_list: Vec<FaderPage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaderPage {
    pub faders: EnumMap<Fader, FaderSources>,
}

impl Default for FaderPage {
    fn default() -> Self {
        Self {
            faders: enum_map! {
                Fader::A => FaderSources::Microphone,
                Fader::B => FaderSources::Chat,
                Fader::C => FaderSources::Music,
                Fader::D => FaderSources::System
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
    pub mute_actions: EnumMap<MuteAction, Vec<OutputChannels>>,

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

impl From<ButtonColourSet> for TwoColour {
    fn from(value: ButtonColourSet) -> Self {
        TwoColour {
            colour1: value.active_colour,
            colour2: value.inactive_colour,
        }
    }
}

/// Colour's related to the Fader Slider
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FaderColourSet {
    /// The colour displayed above the fader
    pub top_colour: Colour,

    /// The colour displayed below the fader
    pub bottom_colour: Colour,
}

impl From<FaderColourSet> for FaderColour {
    fn from(value: FaderColourSet) -> Self {
        FaderColour {
            colour1: value.top_colour,
            colour2: value.bottom_colour,
        }
    }
}

/// These are the different methods of interacting with Mute Keys
#[derive(Debug, Copy, Clone, Enum, Serialize, Deserialize)]
pub enum MuteAction {
    Press,
    Hold,
}

impl From<MuteState> for MuteAction {
    fn from(value: MuteState) -> Self {
        match value {
            MuteState::Unmuted => panic!("Cannot Convert 'Unmuted' to MuteAction"),
            MuteState::Pressed => MuteAction::Press,
            MuteState::Held => MuteAction::Hold,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwearSettings {
    pub volume: u8,
    pub colours: ButtonColourSet,
}

/// This is for handling the cough button and it's settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoughSettings {
    /// The behaviour when pressing the button
    pub cough_behaviour: CoughBehaviour,

    /// The current Channel Assigned to the button (Defaults to Mic)
    pub channel_assignment: FaderSources,

    /// The current channel Mute State
    pub mute_state: MuteState,

    /// Defines what action is performed on Press and Hold
    pub mute_actions: EnumMap<MuteAction, Vec<OutputChannels>>,

    /// Defines the colours and styling of the button
    pub colours: ButtonColourSet,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Enum, Eq, PartialEq)]
pub enum CoughBehaviour {
    Press,
    Hold,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub button_hold_time: u16,
    pub change_page_with_buttons: bool,
}
