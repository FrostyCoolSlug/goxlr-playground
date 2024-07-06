use std::path::PathBuf;

use enum_map::{enum_map, Enum, EnumMap};
use serde::{Deserialize, Serialize};

use goxlr_shared::buttons::InactiveButtonBehaviour;
use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::channels::input::InputChannels;
use goxlr_shared::channels::mute::MuteActionChannels;
use goxlr_shared::channels::output::OutputChannels;
use goxlr_shared::channels::sub_mix::SubMixChannels;
use goxlr_shared::channels::volume::VolumeChannels;
use goxlr_shared::colours::{Colour, FaderColour, FaderDisplayMode, TwoColour};
use goxlr_shared::compressor::{CompressorAttackTime, CompressorRatio, CompressorReleaseTime};
use goxlr_shared::eq_frequencies::{Frequencies, MiniFrequencies};
use goxlr_shared::faders::Fader;
use goxlr_shared::gate::GateTimes;
use goxlr_shared::microphone::MicrophoneType;
use goxlr_shared::mute::MuteState;
use goxlr_shared::submix::Mix;

mod default;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Fader Paging Configuration
    pub pages: FaderPages,

    /// All the Assignable Channels, and their settings..
    pub channels: Channels,

    /// Configuration for the Output Settings..
    pub outputs: EnumMap<OutputChannels, Outputs>,

    /// Configuration for the Swear Button
    pub swear: SwearSettings,

    /// Configuration for the Cough button
    pub cough: CoughSettings,

    /// The Routing Configuration
    pub routing: EnumMap<InputChannels, EnumMap<OutputChannels, bool>>,

    /// The General 'Configuration' of the device
    pub configuration: Configuration,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Outputs {
    /// The Mix this Output is Assigned to when Sub Mixing is enabled
    pub mix_assignment: Mix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaderPages {
    /// The Currently Active Fader Page
    pub current: usize,

    /// A list of all current Fader Pages
    pub page_list: Vec<FaderPage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaderPage {
    /// A map of the Faders, and which Channels are assigned to them on this Page
    pub faders: EnumMap<Fader, FaderChannels>,
}

impl Default for FaderPage {
    fn default() -> Self {
        Self {
            faders: enum_map! {
                Fader::A => FaderChannels::Microphone,
                Fader::B => FaderChannels::Chat,
                Fader::C => FaderChannels::Music,
                Fader::D => FaderChannels::System
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channels {
    /// Volumes for All Channels
    pub volumes: EnumMap<VolumeChannels, u8>,

    /// Configs for Channels which can be assigned to Faders
    pub configs: EnumMap<FaderChannels, FaderChannel>,

    /// Configs for Faders that have configurable Mute Settings
    pub mute_actions: EnumMap<MuteActionChannels, MuteActionChannel>,

    /// Sub-mix Settings for all applicable channels
    pub sub_mix: EnumMap<SubMixChannels, SubMixVolumes>,
}

/// This is a Channel that can be assigned to a fader. All configuration for the channel
/// including colours, mute states and behaviours are configured here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaderChannel {
    /// The current channel Mute State
    pub mute_state: MuteState,

    /// A struct detailing how a fader is displayed on the GoXLR
    pub display: FaderDisplay,
}

/// This is a channel that can have custom mute actions, this generally only applies to inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuteActionChannel {
    /// Defines what action is performed on Press and Hold
    pub mute_actions: EnumMap<MuteAction, Vec<OutputChannels>>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SubMixVolumes {
    /// The Mix B Volumes
    pub volume: u8,

    /// The linked Ratio of mix_a:mix_b
    pub linked: Option<f64>,
}

impl Default for SubMixVolumes {
    fn default() -> Self {
        Self {
            volume: 0,
            linked: Some(1.),
        }
    }
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

    /// The text displayed on the screen (central if no icon, at the bottom if icon)
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
    pub channel_assignment: FaderChannels,

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
    pub submix_enabled: bool,
    pub button_hold_time: u16,
    pub change_page_with_buttons: bool,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct MicProfile {
    pub microphone: Microphone,
    pub equalizer: EnumMap<Frequencies, EqualizerValue>,
    pub equalizer_mini: EnumMap<MiniFrequencies, EqualizerValue>,
    pub compressor: Compressor,
    pub deess: u8,
    pub gate: Gate,
    pub bleep_volume: i8,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Microphone {
    pub mic_type: MicrophoneType,
    pub mic_gains: EnumMap<MicrophoneType, u8>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct EqualizerValue {
    pub gain: i8,
    pub frequency: f32,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Compressor {
    pub threshold: i8,
    pub ratio: CompressorRatio,
    pub attack: CompressorAttackTime,
    pub release: CompressorReleaseTime,
    pub makeup_gain: i8,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Gate {
    pub enabled: bool,

    pub threshold: i8,
    pub attack: GateTimes,
    pub release: GateTimes,
    pub attenuation: u8,
}
