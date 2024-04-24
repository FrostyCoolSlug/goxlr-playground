use enum_map::{enum_map, EnumMap};
use strum::IntoEnumIterator;

use goxlr_shared::channels::{InputChannels, MuteState, OutputChannels};
use goxlr_shared::colours::Colour;
use goxlr_shared::compressor::{CompressorAttackTime, CompressorRatio, CompressorReleaseTime};
use goxlr_shared::eq_frequencies::{Frequencies, MiniFrequencies};
use goxlr_shared::faders::FaderSources;
use goxlr_shared::gate::GateTimes;

use crate::{
    ButtonColourSet, Compressor, CoughBehaviour, CoughSettings, EqualizerValue, FaderChannel,
    FaderColourSet, FaderDisplay, FaderPage, FaderPages, FaderVolumes, Gate,
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
            volume: FaderVolumes {
                mix_a: 182,
                mix_b: 182,
                linked: Some(1.),
            },
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };
        let channel2 = FaderChannel {
            volume: FaderVolumes {
                mix_a: 220,
                mix_b: 220,
                linked: Some(1.),
            },
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };
        let channel3 = FaderChannel {
            volume: FaderVolumes {
                mix_a: 126,
                mix_b: 126,
                linked: Some(1.),
            },
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };
        let channel4 = FaderChannel {
            volume: FaderVolumes {
                mix_a: 70,
                mix_b: 70,
                linked: Some(1.),
            },
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };
        let channel5 = FaderChannel {
            volume: FaderVolumes {
                mix_a: 120,
                mix_b: 120,
                linked: Some(1.),
            },
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };
        let channel6 = FaderChannel {
            volume: FaderVolumes {
                mix_a: 212,
                mix_b: 212,
                linked: Some(1.),
            },
            mute_state: MuteState::Unmuted,
            mute_actions: mute_action.clone(),
            display: fader_display.clone(),
        };

        // We're just going to clone this config out to all the channels, these would realistically
        // all be very different..
        let mut channels: EnumMap<FaderSources, FaderChannel> = enum_map! {
                FaderSources::Microphone => channel.clone(),
                FaderSources::Chat  => channel2.clone(),
                FaderSources::Music => channel3.clone(),
                FaderSources::Game => channel4.clone(),
                FaderSources::Console => channel5.clone(),
                FaderSources::LineIn => channel6.clone(),
                FaderSources::System => channel.clone(),
                FaderSources::Sample => channel2.clone(),
                FaderSources::Headphones => channel3.clone(),
                FaderSources::LineOut => channel4.clone(),
                FaderSources::MicrophoneMonitor => channel5.clone(),
        };

        // Bump headphones volume to 100%..
        channels[FaderSources::Headphones].volume.mix_a = 255;
        channels[FaderSources::Microphone].volume.mix_a = 255;
        channels[FaderSources::MicrophoneMonitor].volume.mix_a = 255 / 100 * 70;

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
                Fader::D => FaderSources::MicrophoneMonitor
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

        // Mute Behaviours..
        channels[FaderSources::System].mute_actions[MuteAction::Press] =
            vec![OutputChannels::Headphones, OutputChannels::LineOut];
        channels[FaderSources::System].mute_actions[MuteAction::Hold] =
            vec![OutputChannels::Headphones, OutputChannels::StreamMix];
        channels[FaderSources::Chat].mute_actions[MuteAction::Hold] =
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
            channel_assignment: FaderSources::System,
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
            MicrophoneType::XLR => 55,
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
                threshold: -40,
                attack: GateTimes::Time10ms,
                release: GateTimes::Time200ms,
                attenuation: 100,
            },
            bleep_volume: -10,
        }
    }
}
