mod types;

use enum_map::{enum_map, Enum, EnumMap};
use goxlr_types::ChannelName;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{EnumIter, IntoEnumIterator};

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
    pub fader_a: FaderSources,
    pub fader_b: FaderSources,
    pub fader_c: FaderSources,
    pub fader_d: FaderSources,
}

impl Default for FaderPage {
    fn default() -> Self {
        Self {
            fader_a: FaderSources::Microphone,
            fader_b: FaderSources::Music,
            fader_c: FaderSources::Chat,
            fader_d: FaderSources::System,
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

/// A Simple Colour object containing Red, Green and Blue values, Defaults to Black
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct Colour {
    /// The Red Value
    pub red: u8,

    /// The Green Value
    pub green: u8,

    /// The Blue Value
    pub blue: u8,
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

/// Defines a Fader, from Left to Right, with the leftmost fader being A
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Faders {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    INPUT,
    OUTPUT,
}

/// A list of channels which can be assigned to a fader.
/// Missing From this List: MicMonitor
#[derive(Debug, Copy, Clone, Enum, EnumIter, Serialize, Deserialize)]
pub enum FaderSources {
    Microphone,
    Chat,
    Music,
    Game,
    Console,
    LineIn,
    System,
    Sample,
    Headphones,
    LineOut,
}

/// This struct attaches to Fader Sources to provide mapping to a channel, as well as whether
/// a Fader is an INPUT or and OUTPUT (used when determining Mute behaviour, OUTPUTs can only
/// 'Mute to All'.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct FaderSource {
    pub channel_map: ChannelName,
    channel_type: ChannelType,
}

impl FaderSources {
    pub fn get_type(self) -> ChannelType {
        match self {
            FaderSources::Microphone => ChannelType::INPUT,
            FaderSources::Chat => ChannelType::INPUT,
            FaderSources::Music => ChannelType::INPUT,
            FaderSources::Game => ChannelType::INPUT,
            FaderSources::Console => ChannelType::INPUT,
            FaderSources::LineIn => ChannelType::INPUT,
            FaderSources::System => ChannelType::INPUT,
            FaderSources::Sample => ChannelType::INPUT,
            FaderSources::Headphones => ChannelType::OUTPUT,
            FaderSources::LineOut => ChannelType::OUTPUT,
        }
    }

    /// Responds with a 'FaderSource' defining the mapping to ChannelName and the type of
    /// channel (INPUT / OUTPUT) - TODO, Relevance?
    pub fn get_source(self) -> FaderSource {
        match self {
            FaderSources::Microphone => FaderSource {
                channel_map: ChannelName::Mic,
                channel_type: ChannelType::INPUT,
            },
            FaderSources::Chat => FaderSource {
                channel_map: ChannelName::Chat,
                channel_type: ChannelType::INPUT,
            },
            FaderSources::Music => FaderSource {
                channel_map: ChannelName::Music,
                channel_type: ChannelType::INPUT,
            },
            FaderSources::Game => FaderSource {
                channel_map: ChannelName::Game,
                channel_type: ChannelType::INPUT,
            },
            FaderSources::Console => FaderSource {
                channel_map: ChannelName::Console,
                channel_type: ChannelType::INPUT,
            },
            FaderSources::LineIn => FaderSource {
                channel_map: ChannelName::LineIn,
                channel_type: ChannelType::INPUT,
            },
            FaderSources::System => FaderSource {
                channel_map: ChannelName::System,
                channel_type: ChannelType::INPUT,
            },
            FaderSources::Sample => FaderSource {
                channel_map: ChannelName::Sample,
                channel_type: ChannelType::INPUT,
            },
            FaderSources::Headphones => FaderSource {
                channel_map: ChannelName::Headphones,
                channel_type: ChannelType::OUTPUT,
            },
            FaderSources::LineOut => FaderSource {
                channel_map: ChannelName::LineOut,
                channel_type: ChannelType::OUTPUT,
            },
        }
    }
}

/// A list of channels classified as 'Inputs'
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum InputChannels {
    Microphone,
    Chat,
    Music,
    Game,
    Console,
    LineIn,
    System,
    Sample,
}

/// A list of channels classified as 'Outputs'
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum OutputChannels {
    Headphones,
    StreamMix,
    LineOut,
    ChatMic,
    Sampler,
}

/// These are channels which simply have volume management
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum VolumeChannels {
    MicrophoneMonitor,
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
            fader_a: FaderSources::System,
            fader_b: FaderSources::Game,
            fader_c: FaderSources::LineIn,
            fader_d: FaderSources::LineOut,
        };
        let page3 = FaderPage {
            fader_a: FaderSources::Sample,
            fader_b: FaderSources::Chat,
            fader_c: FaderSources::Console,
            fader_d: FaderSources::Microphone,
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
