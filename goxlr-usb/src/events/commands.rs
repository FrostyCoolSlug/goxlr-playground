use anyhow::Result;
use enum_map::EnumMap;
use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::channels::input::InputChannels;
use goxlr_shared::channels::output::{OutputChannels, RoutingOutput};
use goxlr_shared::channels::volume::VolumeChannels;
use ritelinked::LinkedHashMap;
use tokio::sync::oneshot;

use goxlr_shared::colours::{ColourScheme, FaderDisplayMode};
use goxlr_shared::faders::Fader;
use goxlr_shared::interaction::CurrentStates;
use goxlr_shared::microphone::{MicEffectKeys, MicParamKeys, MicrophoneType};
use goxlr_shared::mute::ChannelMuteState;
use goxlr_shared::routing::RouteValue;
use goxlr_shared::states::ButtonDisplayStates;

/// This is a helper enum for commands that will simply return a Result<()> with no additional
/// data, it helps simplify wrapping these type of commands together.
#[derive(Debug, Clone)]
pub enum BasicResultCommand {
    SetColour(ColourScheme),
    SetVolume(VolumeChannels, u8),
    SetMuteState(ChannelSource, ChannelMuteState),
    AssignFader(Fader, ChannelSource),
    ApplyRouting(InputChannels, EnumMap<RoutingOutput, RouteValue>),
    SetFaderStyle(Fader, Vec<FaderDisplayMode>),
    SetButtonStates(ButtonDisplayStates),
    SetScribble(Fader, [u8; 1024]),

    /// SubMix Stuff
    SetSubMixVolume(ChannelSource, u8),
    SetSubMixMix(Vec<OutputChannels>, Vec<OutputChannels>),

    /// Mic Stuff
    SetMicGain(MicrophoneType, u8),
    SetMicParams(LinkedHashMap<MicParamKeys, f32>),
    SetMicEffects(LinkedHashMap<MicEffectKeys, i32>),
}

#[derive(Debug, Copy, Clone)]
pub enum ChannelSource {
    FromInputChannel(InputChannels),
    FromOutputChannel(OutputChannels),
    FromFaderSource(FaderChannels),
    FromVolumeChannel(VolumeChannels),
}

#[derive(Debug, Copy, Clone)]
pub enum MixOutputChannels {
    FromOutputChannel(OutputChannels),
}

#[derive(Debug)]
pub enum CommandSender {
    GetButtonStates(oneshot::Sender<Result<CurrentStates>>),
    GetMicLevel(oneshot::Sender<Result<f64>>),
    BasicResultCommand(BasicResultCommand, oneshot::Sender<Result<()>>),
}
