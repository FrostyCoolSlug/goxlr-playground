use enum_map::{enum_map, EnumMap};
use strum::IntoEnumIterator;

use goxlr_shared::channels::{InputChannels, OutputChannels};
use goxlr_shared::colours::Colour;
use goxlr_shared::faders::FaderSources;

use crate::MuteAction;
use crate::MuteBehaviour;
use crate::{
    ButtonColourSet, FaderChannel, FaderColourSet, FaderDisplay, FaderPage, FaderPages,
    InactiveButtonBehaviour, MuteState, Profile, Screen,
};
use crate::{Configuration, Fader};

/// The default profile if one isn't found..
/// TODO: This should be more basic, but using advanced stuff for testing..
impl Default for Profile {
    fn default() -> Self {
        // Lets build a profile, gotta start from the bottom up..

        // Configure the Press and Hold behaviours to be 'Mute to All'
        let mute_action = enum_map! {
            MuteAction::Hold => MuteBehaviour {
                mute_targets: vec![]
            },
            MuteAction::Press => MuteBehaviour {
                mute_targets: vec![]
            },
        };

        let display_mode = vec![];

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
            volume: 128,
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action,
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
                FaderSources::MicrophoneMonitor => channel.clone(),
        };

        // Bump headphones volume to 100%..
        channels[FaderSources::Headphones].volume = 255;

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
                FaderSources::MicrophoneMonitor => Colour {
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
                Fader::A => FaderSources::System,
                Fader::B => FaderSources::Game,
                Fader::C => FaderSources::LineIn,
                Fader::D => FaderSources::LineOut
            },
        };
        let page3 = FaderPage {
            faders: enum_map! {
                Fader::A => FaderSources::Sample,
                Fader::B => FaderSources::Chat,
                Fader::C => FaderSources::Console,
                Fader::D => FaderSources::Microphone
            },
        };

        let pages = FaderPages {
            current: 0,
            page_list: vec![page, page2, page3],
        };

        // Default Routing Table (based on old defaults..)
        let mut routing: EnumMap<InputChannels, EnumMap<OutputChannels, bool>> = Default::default();

        // Headphones and Stream Mix go to all..
        for input in InputChannels::iter() {
            routing[input][OutputChannels::Headphones] = true;
            routing[input][OutputChannels::StreamMix] = true;
        }

        // Mic goes to Lineout, Chat Mic and Sampler..
        routing[InputChannels::Microphone][OutputChannels::LineOut] = true;
        routing[InputChannels::Microphone][OutputChannels::ChatMic] = true;
        routing[InputChannels::Microphone][OutputChannels::Sampler] = true;

        // Samples go to Chat Mic..
        routing[InputChannels::Sample][OutputChannels::ChatMic] = true;

        // General Configuration
        let configuration = Configuration {
            button_hold_time: 500,
        };

        Profile {
            channels,
            pages,
            routing,
            configuration,
        }
    }
}
