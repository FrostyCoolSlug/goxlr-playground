use enum_map::{enum_map, EnumMap};
use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::channels::input::InputChannels;
use goxlr_shared::channels::output::OutputChannels;
use goxlr_shared::channels::volume::VolumeChannels;
use strum::IntoEnumIterator;

use goxlr_shared::colours::Colour;
use goxlr_shared::colours::FaderDisplayMode::Meter;
use goxlr_shared::compressor::{CompressorAttackTime, CompressorRatio, CompressorReleaseTime};
use goxlr_shared::eq_frequencies::{Frequencies, MiniFrequencies};

use goxlr_shared::gate::GateTimes;
use goxlr_shared::mute::MuteState;

use crate::{
    ButtonColourSet, Channels, Compressor, CoughBehaviour, CoughSettings, EqualizerValue,
    FaderChannel, FaderColourSet, FaderDisplay, FaderPage, FaderPages, Gate,
    InactiveButtonBehaviour, MicProfile, Microphone, MicrophoneType, Profile, Screen,
};
use crate::{Configuration, Fader};
use crate::{MuteAction, SwearSettings};

/// The default profile if one isn't found..
/// TODO: This should be more basic, but using advanced stuff for testing..
impl Default for Profile {
    fn default() -> Self {
        // Lets build a profile, gotta start from the bottom up..

        // Configure the Press and Hold behaviours to be 'Mute to All'
        let mute_action = enum_map! {
            MuteAction::Hold => vec![],
            MuteAction::Press => vec![],
        };

        let display_mode = vec![Meter];

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

        let volumes = enum_map! {
            VolumeChannels::Microphone  => 255,
            VolumeChannels::LineIn => 255,
            VolumeChannels::Console => 255,
            VolumeChannels::System => 128,
            VolumeChannels::Game => 128,
            VolumeChannels::Chat => 128,
            VolumeChannels::Sample => 255,
            VolumeChannels::Music => 128,
            VolumeChannels::Headphones => 255,
            VolumeChannels::MicrophoneMonitor => 255,
            VolumeChannels::LineOut => 255,
        };

        let channel = FaderChannel {
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };
        let channel2 = FaderChannel {
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };
        let channel3 = FaderChannel {
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };
        let channel4 = FaderChannel {
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };
        let channel5 = FaderChannel {
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };
        let channel6 = FaderChannel {
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };

        // We're just going to clone this config out to all the channels, these would realistically
        // all be very different..
        let mut fader_config: EnumMap<FaderChannels, FaderChannel> = enum_map! {
                FaderChannels::Microphone => channel.clone(),
                FaderChannels::Chat  => channel2.clone(),
                FaderChannels::Music => channel3.clone(),
                FaderChannels::Game => channel4.clone(),
                FaderChannels::Console => channel5.clone(),
                FaderChannels::LineIn => channel6.clone(),
                FaderChannels::System => channel.clone(),
                FaderChannels::Sample => channel2.clone(),
                FaderChannels::Headphones => channel3.clone(),
                FaderChannels::LineOut => channel4.clone(),
        };

        let base_colour: EnumMap<FaderChannels, Colour> = enum_map! {
                FaderChannels::Microphone => Colour {
                    red: 255,
                    green: 246,
                    blue: 84,
                },
                FaderChannels::Chat  => Colour {
                    red: 36,
                    green: 255,
                    blue: 43,

                },
                FaderChannels::Music => Colour {
                    red:42,
                    green: 255,
                    blue: 112,

                },
                FaderChannels::Game => Colour {
                    red: 255,
                    green: 19,
                    blue: 142,

                },
                FaderChannels::Console => Colour {
                    red: 86,
                    green: 14,
                    blue: 255,

                },
                FaderChannels::LineIn => Colour {
                    red: 255,
                    green: 0,
                    blue: 0,

                },
                FaderChannels::System => Colour {
                    red: 0,
                    green: 255,
                    blue: 0,

                },
                FaderChannels::Sample => Colour {
                    red: 0,
                    green: 0,
                    blue: 255,

                },
                FaderChannels::Headphones => Colour {
                    red: 255,
                    green: 36,
                    blue: 13,

                },
                FaderChannels::LineOut => Colour {
                    red: 255,
                    green: 0,
                    blue: 255,
                },
        };

        // In the interests of testing, set the scribble to the name of the channel..
        for channel in FaderChannels::iter() {
            fader_config[channel].display.screen_display.text = Some(format!("{:?}", channel));
            fader_config[channel].display.screen_display.colour = base_colour[channel];
            fader_config[channel].display.mute_colours.active_colour = base_colour[channel];
            fader_config[channel].display.screen_display.colour = base_colour[channel];
            fader_config[channel].display.fader_colours.bottom_colour = base_colour[channel];
        }

        let channels = Channels {
            volumes,
            configs: fader_config.clone(),
            sub_mix: Default::default(),
        };

        let page = FaderPage::default();
        let page2 = FaderPage {
            faders: enum_map! {
                Fader::A => FaderChannels::System,
                Fader::B => FaderChannels::Game,
                Fader::C => FaderChannels::LineIn,
                Fader::D => FaderChannels::LineOut
            },
        };
        let page3 = FaderPage {
            faders: enum_map! {
                Fader::A => FaderChannels::Sample,
                Fader::B => FaderChannels::Chat,
                Fader::C => FaderChannels::Console,
                Fader::D => FaderChannels::Headphones
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

        // Mute Behaviours..
        fader_config[FaderChannels::System].mute_actions[MuteAction::Press] =
            vec![OutputChannels::Headphones, OutputChannels::LineOut];
        fader_config[FaderChannels::System].mute_actions[MuteAction::Hold] =
            vec![OutputChannels::Headphones, OutputChannels::StreamMix];
        fader_config[FaderChannels::Chat].mute_actions[MuteAction::Hold] =
            vec![OutputChannels::StreamMix];

        // General Configuration
        let configuration = Configuration {
            submix_enabled: false,
            change_page_with_buttons: true,
            button_hold_time: 1000,
        };

        let swear = SwearSettings {
            volume: 255,
            colours: ButtonColourSet {
                active_colour: Colour {
                    red: 0,
                    green: 255,
                    blue: 255,
                },
                inactive_colour: Default::default(),
                inactive_behaviour: InactiveButtonBehaviour::DimActive,
            },
        };

        let mute_action = enum_map! {
            MuteAction::Hold => vec![OutputChannels::Headphones],
            MuteAction::Press => vec![OutputChannels::StreamMix],
        };

        let cough = CoughSettings {
            cough_behaviour: CoughBehaviour::Press,
            channel_assignment: FaderChannels::System,
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action,

            colours: ButtonColourSet {
                active_colour: Colour {
                    red: 0,
                    green: 255,
                    blue: 255,
                },
                inactive_colour: Default::default(),
                inactive_behaviour: InactiveButtonBehaviour::DimActive,
            },
        };

        // Set all the Output Mixes to A
        let outputs = Default::default();

        Profile {
            channels,
            outputs,
            pages,
            routing,
            swear,
            cough,
            configuration,
        }
    }
}

impl Default for MicProfile {
    fn default() -> Self {
        let eq = enum_map! {
            Frequencies::Eq31h => EqualizerValue {
                frequency: 31.5,
                gain: 0,
            },
            Frequencies::Eq63h => EqualizerValue {
                frequency: 63.,
                gain: 0
            },
            Frequencies::Eq125h => EqualizerValue {
                frequency: 125.,
                gain: 0,
            },
            Frequencies::Eq250h => EqualizerValue {
                frequency: 250.,
                gain: 0,
            },
            Frequencies::Eq500h => EqualizerValue {
                frequency: 500.,
                gain: 0
            },
            Frequencies::Eq1kh => EqualizerValue {
                frequency: 1000.,
                gain: 0
            },
            Frequencies::Eq2kh => EqualizerValue {
                frequency: 2000.,
                gain: 0
            },
            Frequencies::Eq4kh => EqualizerValue {
                frequency: 4000.,
                gain: 0
            },
            Frequencies::Eq8kh => EqualizerValue {
                frequency: 8000.,
                gain: 0
            },
            Frequencies::Eq16kh => EqualizerValue {
                frequency: 16000.,
                gain: 0
            }
        };

        let eq_mini = enum_map! {
                MiniFrequencies::Eq90h => EqualizerValue {
                frequency: 90.,
                gain: 0
            },
            MiniFrequencies::Eq250h => EqualizerValue {
                frequency: 250.,
                gain: 0
            },
            MiniFrequencies::Eq500h => EqualizerValue {
                frequency: 500.,
                gain: 0
            },
            MiniFrequencies::Eq1kh => EqualizerValue {
                frequency: 1000.,
                gain: 0
            },
            MiniFrequencies::Eq3kh => EqualizerValue {
                frequency: 3000.,
                gain: 0
            },
            MiniFrequencies::Eq8kh => EqualizerValue {
                frequency: 8000.,
                gain: 0
            }
        };

        let gains = enum_map! {
            MicrophoneType::XLR => 45,
            MicrophoneType::Phantom => 35,
            MicrophoneType::Jack => 40,
        };

        MicProfile {
            microphone: Microphone {
                mic_type: MicrophoneType::XLR,
                mic_gains: gains,
            },
            equalizer: eq,
            equalizer_mini: eq_mini,
            compressor: Compressor {
                threshold: 0,
                ratio: CompressorRatio::Ratio3_2,
                attack: CompressorAttackTime::Attack2ms,
                release: CompressorReleaseTime::Release100ms,
                makeup_gain: 0,
            },
            deess: 0,
            gate: Gate {
                enabled: true,
                threshold: -53,
                attack: GateTimes::Time10ms,
                release: GateTimes::Time200ms,
                attenuation: 100,
            },
            bleep_volume: -10,
        }
    }
}
