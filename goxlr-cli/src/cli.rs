use clap::{Parser, Subcommand};
use goxlr_shared::channels::MuteState;
use goxlr_shared::faders::{Fader, FaderSources};

#[derive(Parser, Debug)]
#[command(about, version, author)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Displays the Status information as JSON
    #[arg(long)]
    pub status_json: bool,

    #[command(subcommand)]
    pub(crate) command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Channels {
        #[arg(value_enum)]
        channel: FaderSources,

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
    Volume { volume: u8 },
    Mute { mute_state: MuteState },
}

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
        channel: FaderSources,
    },
}
