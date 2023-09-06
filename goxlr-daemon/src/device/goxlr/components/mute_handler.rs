use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use strum::IntoEnumIterator;

use goxlr_profile::MuteAction;
use goxlr_shared::channels::ChannelMuteState::{Muted, Unmuted};
use goxlr_shared::channels::{
    ChannelMuteState, InputChannels, MuteState, OutputChannels, RoutingOutput,
};
use goxlr_shared::faders::FaderSources;
use goxlr_shared::routing::RouteValue;
use goxlr_shared::states::State;
use goxlr_usb::events::commands::{BasicResultCommand, ChannelSource};

use crate::device::goxlr::components::fader::DeviceFader;
use crate::device::goxlr::components::routing_handler::RoutingHandler;
use crate::device::goxlr::device::GoXLR;

type Source = FaderSources;
type Target = Vec<OutputChannels>;

#[async_trait]
pub(crate) trait MuteHandler {
    /// Used when loading profiles to set the initial state
    async fn set_mute_initial(&mut self, source: Source) -> Result<()>;

    /// Programmatically Setting the mute states..
    async fn set_mute_state(&mut self, source: Source, state: MuteState) -> Result<()>;
    async fn sync_mute_state(&mut self, source: Source) -> Result<()>;

    /// Used for button handling..
    async fn handle_mute_press(&mut self, source: Source) -> Result<()>;
    async fn handle_mute_hold(&mut self, source: Source) -> Result<()>;
    async fn handle_unmute(&mut self, source: Source) -> Result<()>;

    /// Returns the Button state for a mute button..
    fn get_mute_button_state(&mut self, source: Source) -> State;
}

#[async_trait]
impl MuteHandler for GoXLR {
    /// For this method, we assume that all the mute settings are incorrect, and we go through and
    /// update the routing table, and mute states to ensure they match the 'base' level.
    async fn set_mute_initial(&mut self, source: Source) -> Result<()> {
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
                self.mute_to_targets(source, targets).await?;
            }
        }

        Ok(())
    }

    /// This updates / changes the mute state depending on what value was passed in.
    async fn set_mute_state(&mut self, source: Source, state: MuteState) -> Result<()> {
        let current = self.profile.channels[source].mute_state;

        // Same State, nothing to do here.
        if state == current {
            return Ok(());
        }

        // Are we simply unmuting this channel?
        if state == MuteState::Unmuted {
            return self.handle_unmute(source).await;
        }

        // Otherwise, get our targets and send it
        let action = MuteAction::from(state);
        let targets = self.profile.channels[source].mute_actions[action].clone();

        let changes = self.mute_to_targets(source, targets).await?;
        self.apply_mute_changes(changes).await?;

        // Update the button state and return.
        return self.update_mute_state(source, state).await;
    }

    /// This is generally called when either a channels mute target list changes, or there's some
    /// other change to the transient routing. It's goal is to resync the state.
    async fn sync_mute_state(&mut self, source: FaderSources) -> Result<()> {
        todo!()
    }

    /// Code which triggers when a channel is changed to a 'Pressed' state, primarily it'll
    /// either unmute the channel if it's muted, or will mute to targets in the base state.
    async fn handle_mute_press(&mut self, source: Source) -> Result<()> {
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
        let changes = self.mute_to_targets(source, targets).await?;

        self.apply_mute_changes(changes).await?;
        return self.update_mute_state(source, MuteState::Pressed).await;
    }

    /// This is now simple, grab our new targets, and send it.
    async fn handle_mute_hold(&mut self, source: Source) -> Result<()> {
        debug!("Handling Mute Hold for {:?}", source);

        let current_state = self.profile.channels[source].mute_state;
        if current_state == MuteState::Held {
            return Ok(());
        }

        let targets = self.profile.channels[source].mute_actions[MuteAction::Hold].clone();
        let change = self.mute_to_targets(source, targets).await?;

        self.apply_mute_changes(change).await?;
        return self.update_mute_state(source, MuteState::Held).await;
    }

    async fn handle_unmute(&mut self, source: Source) -> Result<()> {
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
    async fn mute_to_targets(&mut self, source: Source, targets: Target) -> Result<MuteChanges>;
    async fn mute_to_all(&mut self, source: Source) -> Result<MuteChanges>;
    async fn unmute(&mut self, source: Source) -> Result<MuteChanges>;

    async fn send_mute_state(&mut self, source: Source, state: ChannelMuteState) -> Result<()>;
    async fn apply_mute_changes(&self, changes: MuteChanges) -> Result<()>;

    fn restore_routing_from_profile(&mut self, source: Source) -> Result<MuteChanges>;
}

#[async_trait]
impl MuteHandlerLocal for GoXLR {
    /// This is a general 'all encompassing' method for handling mute state changes, it verifies
    /// and returns changes to the routing table (where necessary) to match the target
    /// list being passed in.
    async fn mute_to_targets(&mut self, source: Source, targets: Target) -> Result<MuteChanges> {
        if targets.is_empty() {
            // Call the 'Mute to All' code first..
            self.mute_to_all(source).await?;

            // Update the routing to remove any transient routes, this will be empty unless there's
            // an actual change.
            return self.restore_routing_from_profile(source);
        }

        // Ok, new targets incoming, step one, remove all existing transient routes
        let mut restore = self.restore_routing_from_profile(source)?;
        let source = InputChannels::from(source);

        let mut route_change = false;

        // Now, we simply iterate over our targets, and set their new state
        for target in targets {
            let route = RoutingOutput::from(target);

            if self.set_route(source, route, RouteValue::Off)? && !route_change {
                debug!("Activating Transient Mute {:?} to {:?}", source, route);
                route_change = true;
            }
        }

        // If we were updated, send back the response.
        if route_change && !restore.routing.contains(&source) {
            restore.routing.push(source);
        }

        return Ok(restore);
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
            updated_routes = self.restore_routing_from_profile(source)?.routing;
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

    /// This function simply updates the routing table to reset any transient mute states from
    /// the profile, to allow for general cleaning up before other changes.
    fn restore_routing_from_profile(&mut self, source: Source) -> Result<MuteChanges> {
        let mut updated_routes = vec![];

        // Firstly, check if this is a valid routing target..
        if GoXLR::is_valid_routing_target(source) {
            // Grab the Source
            let source = InputChannels::from(source);
            for output in OutputChannels::iter() {
                // Pull the target..
                let route = RoutingOutput::from(output);

                // Get the Value from the Profile, and from the Routing Table
                let profile_value = self.profile.routing[source][output];
                let active = self.routing_state.get_routing(source, route);

                // Because muting will never affect a routing value that's set to 'Off', we don't
                // need to worry too much about handling false here.
                if profile_value {
                    // Compare it against the routing table..
                    match active {
                        RouteValue::Off => {
                            if self.set_route(source, route, RouteValue::On)? {
                                debug!("Removing Transient Mute {:?} to {:?}", source, route);
                                if !updated_routes.contains(&source) {
                                    updated_routes.push(source);
                                }
                            }
                        }
                        _ => {
                            // One way or another, this route is already enabled, and thus
                            // doesn't need any additional interaction.
                        }
                    }
                }
            }
        }

        Ok(MuteChanges {
            routing: updated_routes,
        })
    }
}

/// This structure provides a list of things which have been changed by the mute commands,
/// generally speaking, they'll be followed up by applying them!
#[derive(Default)]
pub(crate) struct MuteChanges {
    routing: Vec<InputChannels>,
}
