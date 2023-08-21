use anyhow::Result;
use enum_map::EnumMap;
use tokio::sync::oneshot;

use goxlr_shared::channels::{InputChannels, OutputChannels, RoutingOutput, VolumeChannels};
use goxlr_shared::colours::ColourScheme;
use goxlr_shared::faders::{Fader, FaderSources};
use goxlr_shared::routing::RouteValue;

/// This is a helper enum for commands that will simply return a Result<()> with no additional
/// data, it helps simplify wrapping these type of commands together.
#[derive(Copy, Clone)]
pub enum BasicResultCommand {
    SetColour(ColourScheme),
    AssignFader(Fader, ChannelSource),
    ApplyRouting(InputChannels, EnumMap<RoutingOutput, RouteValue>),
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
