use anyhow::Result;
use tokio::sync::oneshot;

use goxlr_shared::channels::{InputChannels, OutputChannels, VolumeChannels};
use goxlr_shared::colours::ColourScheme;
use goxlr_shared::faders::{Fader, FaderSources};

/// This is a helper enum for commands that will simply return a Result<()> with no additional
/// data, it helps simplify wrapping these type of commands together.
#[derive(Copy, Clone)]
pub enum BasicResultCommand {
    SetColour(ColourScheme),
    AssignFader(Fader, ChannelSource),
}

#[derive(Copy, Clone)]
pub enum ChannelSource {
    FromInputChannel(InputChannels),
    FromOutputChannel(OutputChannels),
    FromFaderSource(FaderSources),
    FromVolumeChannel(VolumeChannels),
}

pub enum CommandSender {
    BasicResultCommand(BasicResultCommand, oneshot::Sender<Result<()>>),
}
