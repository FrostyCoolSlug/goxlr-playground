use anyhow::Result;
use enum_map::EnumMap;
use tokio::sync::oneshot;

use goxlr_shared::channels::{
    ChannelMuteState, InputChannels, OutputChannels, RoutingOutput, VolumeChannels,
};
use goxlr_shared::colours::{ColourScheme, FaderDisplayMode};
use goxlr_shared::faders::{Fader, FaderSources};
use goxlr_shared::interaction::CurrentStates;
use goxlr_shared::routing::RouteValue;
use goxlr_shared::states::ButtonDisplayStates;

/// This is a helper enum for commands that will simply return a Result<()> with no additional
/// data, it helps simplify wrapping these type of commands together.
#[derive(Clone)]
pub enum BasicResultCommand {
    SetColour(ColourScheme),
    SetVolume(ChannelSource, u8),
    SetMuteState(ChannelSource, ChannelMuteState),
    AssignFader(Fader, ChannelSource),
    ApplyRouting(InputChannels, EnumMap<RoutingOutput, RouteValue>),
    SetFaderStyle(Fader, Vec<FaderDisplayMode>),
    SetButtonStates(ButtonDisplayStates),
    SetScribble(Fader, [u8; 1024]),
}

#[derive(Copy, Clone)]
pub enum ChannelSource {
    FromInputChannel(InputChannels),
    FromOutputChannel(OutputChannels),
    FromFaderSource(FaderSources),
    FromVolumeChannel(VolumeChannels),
}

pub enum CommandSender {
    GetButtonStates(oneshot::Sender<Result<CurrentStates>>),
    GetMicLevel(oneshot::Sender<Result<f64>>),
    BasicResultCommand(BasicResultCommand, oneshot::Sender<Result<()>>),
}
