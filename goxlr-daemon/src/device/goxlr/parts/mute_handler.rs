use anyhow::{bail, Result};
use async_trait::async_trait;
use log::debug;
use strum::IntoEnumIterator;

use goxlr_profile::{MuteAction, MuteState};
use goxlr_shared::buttons::InactiveButtonBehaviour;
use goxlr_shared::channels::ChannelMuteState::{Muted, Unmuted};
use goxlr_shared::channels::{ChannelMuteState, InputChannels, OutputChannels, RoutingOutput};
use goxlr_shared::faders::FaderSources;
use goxlr_shared::routing::RouteValue;
use goxlr_shared::states::State;
use goxlr_shared::states::State::{Colour2, DimmedColour1, DimmedColour2};
use goxlr_usb::events::commands::{BasicResultCommand, ChannelSource};

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::parts::fader::DeviceFader;
use crate::device::goxlr::parts::routing_handler::RoutingHandler;

type Source = FaderSources;
type Target = Vec<OutputChannels>;

#[async_trait]
pub(crate) trait MuteHandler {
    /// Used when loading profiles to set the initial state
    async fn handle_mute_initial(&mut self, source: FaderSources) -> Result<()>;

    /// Used for button handling..
    async fn handle_mute_press(&mut self, source: FaderSources) -> Result<()>;
    async fn handle_mute_hold(&mut self, source: FaderSources) -> Result<()>;
    async fn handle_unmute(&mut self, source: FaderSources) -> Result<()>;

    /// Returns the Button state for a mute button..
    fn get_mute_button_state(&mut self, source: Source) -> State;
}

#[async_trait]
impl MuteHandler for GoXLR {
    /// For this method, we assume that all the mute settings are incorrect, and we go through and
    /// update the routing table, and mute states to ensure they match the 'base' level.
    async fn handle_mute_initial(&mut self, source: FaderSources) -> Result<()> {
        let state = self.profile.channels[source].mute_state;
        match state {
            MuteState::Unmuted => {
                // This one is pretty trivial, we can just call 'unmute', which should set the
                // routing table, and mute state back to their defaults from the profile.
                self.unmute(source).await?;
            }
            MuteState::Pressed | MuteState::Held => {
                let action = MuteAction::from(state);

                // We need the targets..
                let targets = self.profile.channels[source].mute_actions[action].clone();

                // We can assume that if this is an initial pass, the routing is currently clean.
                self.mute_to_target(source, targets).await?;
            }
        }

        Ok(())
    }

    /// Code which triggers when a channel is changed to a 'Pressed' state, primarily it'll
    /// either unmute the channel if it's muted, or will mute to targets in the base state.
    async fn handle_mute_press(&mut self, source: FaderSources) -> Result<()> {
        debug!("Handling Mute Press for {:?}", source);

        let current = self.profile.channels[source].mute_state;
        if current != MuteState::Unmuted {
            debug!("{:?} currently muted, unmuting..", source);
            let changes = self.unmute(source).await?;
            self.apply_mute_changes(changes).await?;
            return self.update_mute_state(source, MuteState::Unmuted).await;
        }

        debug!("Channel {:?} not muted, muting", source);
        let targets = self.profile.channels[source].mute_actions[MuteAction::Press].clone();
        let changes = self.mute_to_target(source, targets).await?;

        self.apply_mute_changes(changes).await?;
        return self.update_mute_state(source, MuteState::Pressed).await;
    }

    /// This one is a little more complicated, we could be going from Nothing -> Held,
    /// Pressed -> Held, or even Held -> Held (do nothing).
    async fn handle_mute_hold(&mut self, source: FaderSources) -> Result<()> {
        debug!("Handling Mute Hold for {:?}", source);

        match self.profile.channels[source].mute_state {
            MuteState::Unmuted => {
                debug!("{:?} currently unmuted, doing simple Mute", source);

                // This one is simple, do the same as press except with Hold Targets..
                let targets = self.profile.channels[source].mute_actions[MuteAction::Hold].clone();
                let changes = self.mute_to_target(source, targets).await?;

                // Update the profile state, and apply changes..
                self.profile.channels[source].mute_state = MuteState::Held;
                self.apply_mute_changes(changes).await?;
                return self.update_mute_state(source, MuteState::Held).await;
            }
            MuteState::Pressed => {
                debug!("{:?} In pressed state, updating to Hold", source);

                let current = self.profile.channels[source].mute_actions[MuteAction::Press].clone();
                let targets = self.profile.channels[source].mute_actions[MuteAction::Hold].clone();

                if current.is_empty() && targets.is_empty() {
                    // Both are 'Mute to All', there's nothing to do here..
                    debug!("Both Press and Hold are 'Mute to All', doing nothing");
                    return self.update_mute_state(source, MuteState::Held).await;
                }

                // We can simply call 'Unmute' to reset the routing table to it's base state..
                debug!("Reverting 'Press' State to profile");
                let unmute_change = self.unmute(source).await?;

                // Now call mute to apply the new settings..
                debug!("Applying mute to new target list: {:#?}", targets);
                let mute_change = self.mute_to_target(source, targets).await?;

                debug!("Applying Routing Changes");
                let mut channels_handled = vec![];
                for channel in unmute_change.routing {
                    channels_handled.push(channel);
                    self.apply_routing_for_channel(channel).await?;
                }

                for channel in mute_change.routing {
                    if !channels_handled.contains(&channel) {
                        self.apply_routing_for_channel(channel).await?;
                    }
                }

                return self.update_mute_state(source, MuteState::Held).await;
            }
            MuteState::Held => {
                debug!("{:?} is already in held state, updating colour.", source);
                return self.update_mute_state(source, MuteState::Held).await;
            }
        }
    }

    async fn handle_unmute(&mut self, source: FaderSources) -> Result<()> {
        let changes = self.unmute(source).await?;
        self.apply_mute_changes(changes).await?;
        return self.update_mute_state(source, MuteState::Unmuted).await;
    }

    fn get_mute_button_state(&mut self, source: Source) -> State {
        let channel = self.profile.channels[source].clone();

        match channel.mute_state {
            MuteState::Unmuted => State::from(channel.display.mute_colours.inactive_behaviour),
            MuteState::Pressed => State::Colour1,
            MuteState::Held => State::Blinking,
        }
    }
}

#[async_trait]
trait MuteHandlerLocal {
    async fn mute_to_target(&mut self, source: Source, targets: Target) -> Result<MuteChanges>;
    async fn mute_to_all(&mut self, source: Source) -> Result<MuteChanges>;
    async fn unmute(&mut self, source: Source) -> Result<MuteChanges>;

    async fn send_mute_state(&mut self, source: Source, state: ChannelMuteState) -> Result<()>;
    async fn apply_mute_changes(&self, changes: MuteChanges) -> Result<()>;
}

#[async_trait]
impl MuteHandlerLocal for GoXLR {
    async fn mute_to_target(&mut self, source: Source, targets: Target) -> Result<MuteChanges> {
        // If our target list is empty, activate 'mute to all' behaviour..
        if targets.is_empty() {
            debug!("Target List Empty for {:?}, Muting to All", source);
            return self.mute_to_all(source).await;
        }

        // Make sure this channel is capable of route based muting..
        if !GoXLR::is_valid_routing_target(source) {
            bail!("Cannot Apply Target Routing to this Channel!");
        }

        debug!("Applying Target Routing for {:?}", source);
        debug!("Targets: {:?}", targets);
        let mut routing_updated = false;

        // This is relatively straight forward, go through the targets for this source, and set
        // their routing to 'Off'.
        for target in targets {
            let source = InputChannels::from(source);
            let target = RoutingOutput::from(target);

            debug!("Transient disable route {:?} to {:?}", source, target);
            let change = self.set_route(source, target, RouteValue::Off)?;
            if !routing_updated && change {
                routing_updated = true;
            }
        }

        let routing = if routing_updated {
            debug!("Routing has been updated for {:?}", source);
            vec![InputChannels::from(source)]
        } else {
            debug!("No Routing Change from Mute for {:?}", source);
            vec![]
        };

        let changes = MuteChanges { routing };

        Ok(changes)
    }

    async fn mute_to_all(&mut self, source: Source) -> Result<MuteChanges> {
        debug!("Muting Channel {:?} to All", source);
        self.send_mute_state(source, Muted).await?;
        Ok(Default::default())
    }

    async fn unmute(&mut self, source: Source) -> Result<MuteChanges> {
        debug!("Unmuting Channel: {:?}", source);

        let mut updated_routes = vec![];

        // Outputs need to be unmuted, but they also can't be routed.
        if GoXLR::is_valid_routing_target(source) {
            for output in OutputChannels::iter() {
                let source = InputChannels::from(source);
                let route = RoutingOutput::from(output);

                let profile_value = self.profile.routing[source][output];
                let active = self.routing_state.get_routing(source, route);

                // First check to see if this is 'Unmuted by Value'
                if profile_value {
                    match active {
                        RouteValue::Off => {
                            debug!("Removing Transient Route {:?} to {:?}", source, route);
                            self.set_route(source, route, RouteValue::On)?;
                            updated_routes.push(source);
                        }
                        RouteValue::On | RouteValue::Value(_) => {
                            // This route is already currently active, or set to a custom volume,
                            // there's nothing more to do here.
                        }
                    }
                }
            }
        }

        /*
           TODO: Handle the Cough Button.
           This code will completely restore the Microphone to it's natural 'Unmuted' state,
           once complete, we should be able to just re-call the Cough Mute action to restore any
           state muted from there.
        */

        // Don't unmute on a channel which isn't flagged as muted..
        debug!("Unmuting channel {:?}", source);
        self.send_mute_state(source, Unmuted).await?;

        Ok(MuteChanges {
            routing: updated_routes,
        })
    }

    async fn send_mute_state(&mut self, source: Source, state: ChannelMuteState) -> Result<()> {
        // Prepare the GoXLR Command..
        let command_source = ChannelSource::FromFaderSource(source);
        let command = BasicResultCommand::SetMuteState(command_source, state);

        // Check our existing mute state map, to see if we're changing it..
        if let Some(current_state) = self.mute_state[source] {
            // If the requested change is different from our current known state, send the
            // new state to the GoXLR.
            if current_state != state {
                debug!("Setting {:?} to {:?}", source, state);
                self.send_no_result(command).await?;
            } else {
                debug!("Channel {:?} already {:?}, doing nothing.", source, state);
            }
        } else {
            // We don't know the active mute state, so send unmute regardless.
            self.send_no_result(command).await?;
        };

        // Either way, replace the mute state in the struct with our new state.
        self.mute_state[source].replace(state);
        Ok(())
    }

    async fn apply_mute_changes(&self, changes: MuteChanges) -> Result<()> {
        for channel in changes.routing {
            self.apply_routing_for_channel(channel).await?;
        }
        Ok(())
    }
}

/// This structure provides a list of things which have been changed by the mute commands,
/// generally speaking, they'll be followed up by applying them!
#[derive(Default)]
pub(crate) struct MuteChanges {
    routing: Vec<InputChannels>,
}
