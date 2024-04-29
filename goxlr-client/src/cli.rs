use clap::{Parser, Subcommand};
use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::channels::sub_mix::SubMixChannels;
use goxlr_shared::channels::volume::VolumeChannels;
use goxlr_shared::compressor::{CompressorAttackTime, CompressorRatio, CompressorReleaseTime};
use goxlr_shared::eq_frequencies::{Frequencies, MiniFrequencies};
use goxlr_shared::faders::Fader;
use goxlr_shared::gate::GateTimes;
use goxlr_shared::microphone::MicrophoneType;
use goxlr_shared::mute::MuteState;

#[derive(Parser, Debug)]
#[command(about, version, author)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Optional Device Serial
    pub serial: Option<String>,

    /// Displays the Status information as JSON
    #[arg(long)]
    pub status_json: bool,

    #[command(subcommand)]
    pub(crate) command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Microphone {
        #[command(subcommand)]
        command: MicrophoneCommands,
    },

    Channels {
        #[command(subcommand)]
        command: ChannelCommands,
    },

    Pages {
        #[command(subcommand)]
        command: PageCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum ChannelCommands {
    Volumes {
        #[arg(value_enum)]
        channel: VolumeChannels,

        #[command(subcommand)]
        command: VolumeCommands,
    },

    Faders {
        #[arg(value_enum)]
        channel: FaderChannels,

        #[command(subcommand)]
        command: FaderCommands,
    },

    SubMix {
        #[arg(value_enum)]
        channel: SubMixChannels,

        #[command(subcommand)]
        command: SubMixCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum VolumeCommands {
    Volume { volume: u8 },
}

#[derive(Debug, Subcommand)]
pub enum FaderCommands {
    Mute { state: MuteState },
}

#[derive(Debug, Subcommand)]
pub enum SubMixCommands {}

#[derive(Debug, Subcommand)]
pub enum PageCommands {
    SetPage {
        page_number: u8,
    },
    AddPage,
    RemovePage {
        page_number: u8,
    },
    SetFader {
        page_number: u8,
        fader: Fader,
        channel: FaderChannels,
    },
}

#[derive(Debug, Subcommand)]
pub enum MicrophoneCommands {
    SetUp {
        #[command(subcommand)]
        command: MicrophoneSetupCommands,
    },

    Equaliser {
        #[command(subcommand)]
        command: MicrophoneEqCommands,
    },

    Compressor {
        #[command(subcommand)]
        command: MicrophoneCompressorCommands,
    },

    Gate {
        #[command(subcommand)]
        command: MicrophoneGateCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum MicrophoneSetupCommands {
    MicType {
        #[arg(value_enum)]
        microphone_type: MicrophoneType,
    },
    MicGain {
        gain: u8,
    },
}

#[derive(Debug, Subcommand)]
pub enum MicrophoneCompressorCommands {
    Threshold {
        threshold: i8,
    },
    Ratio {
        #[arg(value_enum)]
        ratio: CompressorRatio,
    },
    Attack {
        #[arg(value_enum)]
        attack: CompressorAttackTime,
    },
    Release {
        #[arg(value_enum)]
        release: CompressorReleaseTime,
    },
    MakupGain {
        gain: i8,
    },
}

#[derive(Debug, Subcommand)]
pub enum MicrophoneGateCommands {
    Enabled {
        enabled: bool,
    },
    Threshold {
        threshold: i8,
    },
    Attack {
        #[arg(value_enum)]
        attack: GateTimes,
    },
    Release {
        #[arg(value_enum)]
        release: GateTimes,
    },
    Attenuation {
        attenuation: u8,
    },
}

#[derive(Debug, Subcommand)]
pub enum MicrophoneEqCommands {
    Full {
        #[command(subcommand)]
        command: MicrophoneEqFullCommands,
    },

    Mini {
        #[command(subcommand)]
        command: MicrophoneEqMiniCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum MicrophoneEqMiniCommands {
    Frequency {
        #[arg(value_enum)]
        base: MiniFrequencies,
        frequency: f32,
    },

    Gain {
        #[arg(value_enum)]
        base: MiniFrequencies,
        gain: i8,
    },
}

#[derive(Debug, Subcommand)]
pub enum MicrophoneEqFullCommands {
    Frequency {
        #[arg(value_enum)]
        base: Frequencies,
        frequency: f32,
    },

    Gain {
        #[arg(value_enum)]
        base: Frequencies,
        gain: i8,
    },
}
