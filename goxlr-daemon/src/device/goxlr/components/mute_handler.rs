use anyhow::Result;
use log::debug;
use ritelinked::LinkedHashMap;
use strum::IntoEnumIterator;

use goxlr_profile::MuteAction;
use goxlr_shared::buttons::Buttons::CoughButton;
use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::channels::input::InputChannels;
use goxlr_shared::channels::mute::MuteActionChannels;
use goxlr_shared::channels::output::{OutputChannels, RoutingOutput};
use goxlr_shared::channels::CanFrom;
use goxlr_shared::microphone::MicEffectKeys;
use goxlr_shared::mute::ChannelMuteState::{Muted, Unmuted};
use goxlr_shared::mute::{ChannelMuteState, MuteState};
use goxlr_shared::routing::RouteValue;
use goxlr_shared::states::State;
use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::components::buttons::ButtonHandlers;
use crate::device::goxlr::components::fader::DeviceFader;
use crate::device::goxlr::components::routing_handler::RoutingHandler;
use crate::device::goxlr::device::GoXLR;

type Source = FaderChannels;
type MuteSource = MuteActionChannels;
type Target = Vec<OutputChannels>;

pub(crate) trait MuteHandler {
    /// Programmatically Setting the mute states..
    async fn set_mute_state(&mut self, source: Source, state: MuteState) -> Result<()>;
    //async fn sync_mute_state(&mut self, source: Source) -> Result<()>;

    /// Used for button handling..
    async fn handle_mute_press(&mut self, source: Source) -> Result<()>;
    async fn handle_mute_hold(&mut self, source: Source) -> Result<(bool, bool)>;
    async fn handle_unmute(&mut self, source: Source) -> Result<()>;

    /// Used for the Cough Buttons..
    async fn handle_cough_press(&mut self, hold: bool) -> Result<()>;

    /// Returns the Button state for a mute button..
    fn get_mute_button_state(&self, source: Source) -> State;

    /// Returns the current state of the Cough button..
    fn get_cough_button_state(&self) -> State;

    /// Returns whether a current source is 'Muted to All'
    fn is_muted_to_all(&self, source: Source) -> bool;
}

impl MuteHandler for GoXLR {
    /// This updates / changes the mute state depending on what value was passed in.
    async fn set_mute_state(&mut self, source: Source, state: MuteState) -> Result<()> {
        //let current = self.profile.channels[source].mute_state;

        // Are we simply unmuting this channel?
        if state == MuteState::Unmuted {
            // Maintain 'None' if this channel doesn't support mute Targets
            let channels = match MuteSource::can_from(source) {
                true => self.add_cough_mute(MuteSource::from(source), None),
                false => None,
            };

            // We need to update the lighting regardless, but also need to maintain the cough filter
            if let Some(channels) = channels {
                let mut changes = None;
                if !channels.is_empty() {
                    // Trigger an Unmute, just in case..
                    changes = Some(self.unmute(source).await?);
                }

                // If we unmuted above, we need to persist the Changes going forwards
                let change = if let Some(channel) = changes {
                    let mut second = self.mute_to_targets(source, channels).await?;
                    channel.routing.iter().for_each(|channel| {
                        if !second.routing.contains(channel) {
                            second.routing.push(*channel);
                        }
                    });
                    second
                } else {
                    self.mute_to_targets(source, channels).await?
                };

                self.apply_mute_changes(change).await?;
            } else {
                let changes = self.unmute(source).await?;
                self.apply_mute_changes(changes).await?;
                return Ok(());
            }

            return Ok(());
        }

        // Otherwise, get our targets and send it
        let action = MuteAction::from(state);

        let targets = match MuteSource::can_from(source) {
            true => {
                let source = source.into();
                let base = self.profile.channels.mute_actions[source].actions[action].clone();
                self.add_cough_mute(source, Some(base)).unwrap_or_default()
            }
            false => vec![],
        };

        let changes = self.mute_to_targets(source, targets).await?;
        self.apply_mute_changes(changes).await?;
        self.update_mute_state(source, state).await?;

        Ok(())
    }

    /// This is generally called when either a channels mute target list changes, or there's some
    /// other change to the transient routing. It's goal is to resync the state.
    // async fn sync_mute_state(&mut self, source: FaderSources) -> Result<()> {
    //     todo!()
    // }

    /// Code which triggers when a channel is changed to a 'Pressed' state, primarily it'll
    /// either unmute the channel if it's muted, or will mute to targets in the base state.
    async fn handle_mute_press(&mut self, source: Source) -> Result<()> {
        debug!("Handling Mute Press for {:?}", source);

        let current = self.profile.channels.configs[source].mute_state;
        if current != MuteState::Unmuted {
            return self.handle_unmute(source).await;
        }

        debug!("Channel {:?} not muted, muting", source);
        let action = MuteAction::Press;
        let targets = match MuteSource::can_from(source) {
            true => self.profile.channels.mute_actions[source.into()].actions[action].clone(),
            false => vec![],
        };

        let changes = self.mute_to_targets(source, targets).await?;

        self.apply_mute_changes(changes).await?;
        self.update_mute_state(source, MuteState::Pressed).await
    }

    /// This is now simple, grab our new targets, and send it.
    async fn handle_mute_hold(&mut self, source: Source) -> Result<(bool, bool)> {
        debug!("Handling Mute Hold for {:?}", source);

        if !MuteActionChannels::can_from(source) {
            // This channel only supports muting to all, so we flag the 'Hold' as handled which,
            // prevents fader paging kicking in, but also get told about the release.
            return Ok((true, false));
        }

        let current_state = self.profile.channels.configs[source].mute_state;
        if current_state == MuteState::Held {
            // We're already in a held state, flag as success and skip the release
            return Ok((true, true));
        }

        let action = MuteAction::Hold;
        let targets = match MuteSource::can_from(source) {
            true => self.profile.channels.mute_actions[source.into()].actions[action].clone(),
            false => vec![],
        };

        let change = self.mute_to_targets(source, targets).await?;

        self.apply_mute_changes(change).await?;
        self.update_mute_state(source, MuteState::Held).await?;

        // We are done, flag handled and skip the release
        Ok((true, true))
    }

    async fn handle_unmute(&mut self, source: Source) -> Result<()> {
        // We only need to do something if this channel is actually muted..
        let current = self.profile.channels.configs[source].mute_state;
        if current != MuteState::Unmuted {
            debug!("{:?} currently muted, handling unmute..", source);

            // Before we 'Unmute', double-check the Cough Button..
            let targets = match MuteSource::can_from(source) {
                true => self.add_cough_mute(MuteSource::from(source), None),
                false => None,
            };

            let changes = if let Some(targets) = targets {
                debug!("Restoring Cough Mute State..");
                self.mute_to_targets(source, targets).await?
            } else {
                self.unmute(source).await?
            };

            self.apply_mute_changes(changes).await?;
            return self.update_mute_state(source, MuteState::Unmuted).await;
        } else {
            debug!("{:?} already unmuted, doing nothing!", source);
        }
        Ok(())
    }

    async fn handle_cough_press(&mut self, hold: bool) -> Result<()> {
        // This is relatively simple, we simply re-process the channel we're attached to..
        let current = self.profile.cough.mute_state;
        let cough_source = self.profile.cough.channel_assignment;

        // Get the current mute state of this channel..
        let channel_state = self.profile.channels.configs[cough_source.into()].mute_state;

        // Update the state of the Mute Button..
        if hold && current != MuteState::Held {
            debug!("Cough Held, Handling..");
            self.profile.cough.mute_state = MuteState::Held;
        } else if current == MuteState::Unmuted {
            debug!("Cough Unmuted, Muting to X");
            self.profile.cough.mute_state = MuteState::Pressed;
        } else {
            debug!("Cough Muted, Unmuting..");
            self.profile.cough.mute_state = MuteState::Unmuted;
        }

        // Resync the state of the channel
        let source = cough_source.into();
        self.set_mute_state(source, channel_state).await?;

        let cough_state = self.get_cough_button_state();
        self.button_states.set_state(CoughButton, cough_state);
        self.apply_button_states().await?;

        Ok(())
    }

    fn get_mute_button_state(&self, source: Source) -> State {
        let channel = self.profile.channels.configs[source].clone();

        match channel.mute_state {
            MuteState::Unmuted => State::from(channel.display.mute_colours.inactive_behaviour),
            MuteState::Pressed => State::Colour1,
            MuteState::Held => State::Blinking,
        }
    }

    fn get_cough_button_state(&self) -> State {
        match self.profile.cough.mute_state {
            MuteState::Unmuted => State::from(self.profile.cough.colours.inactive_behaviour),
            MuteState::Pressed => State::Colour1,
            MuteState::Held => State::Blinking,
        }
    }

    /// This is a simple call to determine whether our channel is muted to all
    fn is_muted_to_all(&self, source: Source) -> bool {
        let state = self.profile.channels.configs[source].mute_state;

        // Firstly, if we're considered Unmuted, we need to check the cough button to ensure that
        // it's not currently muting us, and we only need to do that if can can cast to MuteSource
        if state == MuteState::Unmuted {
            return match MuteSource::can_from(source) {
                true => match self.add_cough_mute(source.into(), None) {
                    Some(targets) => targets.is_empty(),
                    None => false,
                },
                false => false,
            };
        }

        // If we get here, we're either Pressed or Held, in the case of non MuteSources this
        // means Muted to All by default, otherwise get targets
        match MuteSource::can_from(source) {
            true => {
                // Recast these to the correct types
                let source = source.into();
                let state = state.into();

                // Send it along and check the vec size
                self.get_targets_for_action(source, state).is_empty()
            }

            // Force Mute To All for these channels
            false => true,
        }
    }
}

pub(crate) trait MuteHandlerCrate {
    /// Used when loading profiles to set the initial state
    async fn set_mute_initial(&mut self, source: Source) -> Result<()>;
}

impl MuteHandlerCrate for GoXLR {
    /// For this method, we assume that all the mute settings are incorrect, and we go through and
    /// update the routing table, and mute states to ensure they match the 'base' level.
    async fn set_mute_initial(&mut self, source: Source) -> Result<()> {
        let state = self.profile.channels.configs[source].mute_state;
        match state {
            MuteState::Unmuted => {
                let targets = match MuteSource::can_from(source) {
                    true => self.add_cough_mute(source.into(), None),
                    false => None,
                };

                if let Some(targets) = targets {
                    // If the target list isn't blank, we're muting to 'some', but not all.
                    // Force a channel unmute to the device before applying the settings
                    if !targets.is_empty() {
                        self.unmute(source).await?;
                    }

                    // Now apply the mute settings..
                    let changes = self.mute_to_targets(source, targets).await?;
                    self.apply_mute_changes(changes).await?;
                } else {
                    let changes = self.unmute(source).await?;
                    self.apply_mute_changes(changes).await?;
                }
            }
            MuteState::Pressed | MuteState::Held => {
                // Same applies here, we don't know the muted state of the device..
                let action = MuteAction::from(state);
                let channel = MuteSource::from(source);
                let targets = match MuteSource::can_from(source) {
                    true => self.profile.channels.mute_actions[channel].actions[action].clone(),
                    false => vec![],
                };

                // Should we unmute existing channels?
                if !targets.is_empty() {
                    self.unmute(source).await?;
                }

                // Then mute to targets..
                let changes = self.mute_to_targets(source, targets).await?;
                self.apply_mute_changes(changes).await?;
            }
        }

        Ok(())
    }
}

trait MuteHandlerLocal {
    async fn mute_to_targets(&mut self, source: Source, targets: Target) -> Result<MuteChanges>;
    async fn mute_to_all(&mut self, source: Source) -> Result<MuteChanges>;
    async fn unmute(&mut self, source: Source) -> Result<MuteChanges>;

    async fn send_mute_state(&mut self, source: Source, state: ChannelMuteState) -> Result<()>;
    async fn send_mic_mute_state(&self, muted: bool) -> Result<()>;
    async fn apply_mute_changes(&self, changes: MuteChanges) -> Result<()>;

    fn get_targets_for_action(&self, source: MuteSource, mute_action: MuteAction) -> Target;
    fn add_cough_mute(&self, source: MuteSource, current: Option<Target>) -> Option<Target>;
    fn restore_routing_from_profile(&mut self, source: Source) -> Result<MuteChanges>;
}

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

        // Grab the current routing row for this source..
        let original = self.get_routing_input_row(source.into());

        let mut restore = self.restore_routing_from_profile(source)?;
        let source = source.into();

        let mut route_change = false;

        // Now, we simply iterate over our targets, and set their new state
        for target in targets {
            let route = target.into();

            if self.set_route(source, route, RouteValue::Off)? && !route_change {
                debug!("Activating Transient Mute {:?} to {:?}", source, route);
                route_change = true;
            }
        }

        let new = self.get_routing_input_row(source);

        if original == new {
            debug!("No Change to Routing Row, returning empty");
            return Ok(MuteChanges::default());
        }

        // If we were updated, send back the response.
        if route_change && !restore.routing.contains(&source) {
            restore.routing.push(source);
        }

        Ok(restore)
    }

    async fn mute_to_all(&mut self, source: Source) -> Result<MuteChanges> {
        debug!("Muting Channel {:?} to All", source);

        // The Microphone also has an 'Effect' which needs to be set when muting / unmuting
        if source == FaderChannels::Microphone {
            self.send_mic_mute_state(true).await?;
        }

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

        // The Microphone also has an 'Effect' which needs to be set when muting / unmuting
        if source == FaderChannels::Microphone {
            self.send_mic_mute_state(false).await?;
        }

        // Don't unmute on a channel which isn't flagged as muted..
        debug!("Checking device state for {:?}", source);
        self.send_mute_state(source, Unmuted).await?;

        Ok(MuteChanges {
            routing: updated_routes,
        })
    }

    async fn send_mute_state(&mut self, source: Source, state: ChannelMuteState) -> Result<()> {
        // Prepare the GoXLR Command..
        let command = BasicResultCommand::SetMuteState(source, state);

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

    async fn send_mic_mute_state(&self, muted: bool) -> Result<()> {
        let map = LinkedHashMap::from_iter([(MicEffectKeys::MicInputMute, muted as i32)]);
        let command = BasicResultCommand::SetMicEffects(map);
        self.send_no_result(command).await
    }

    async fn apply_mute_changes(&self, changes: MuteChanges) -> Result<()> {
        for channel in changes.routing {
            self.apply_routing_for_channel(channel).await?;
        }
        Ok(())
    }

    fn get_targets_for_action(&self, source: MuteSource, mute_action: MuteAction) -> Target {
        let source = source.into();
        let targets = self.profile.channels.mute_actions[source].actions[mute_action].clone();

        // Apply the Cough Button Settings (if needed)
        let cough_targets = self.add_cough_mute(source, Some(targets.clone()));
        if let Some(targets) = cough_targets {
            targets
        } else {
            targets
        }
    }

    fn add_cough_mute(&self, source: MuteSource, current: Option<Target>) -> Option<Target> {
        let cough_source = self.profile.cough.channel_assignment;
        let cough_state = self.profile.cough.mute_state;

        if cough_source != source || cough_state == MuteState::Unmuted {
            // We're not attached to this source, nor are we muted, nothing to do here.
            return None;
        }

        // Ok, we need to adjust our target list to correctly match.
        let cough_action = MuteAction::from(cough_state);
        let cough_targets = self.profile.cough.mute_actions[cough_action].clone();

        return match current.clone() {
            None => {
                // No targets passed in, all we need is the current target list
                Some(cough_targets)
            }
            Some(channels) => {
                // If either of our lists are empty, return empty (Mute to All)
                if channels.is_empty() || cough_targets.is_empty() {
                    return Some(vec![]);
                }

                // Build a list containing targets for both source, and cough..
                let current = current.unwrap();
                let mut final_targets = current.clone();
                cough_targets.iter().for_each(|target| {
                    if !final_targets.contains(target) {
                        final_targets.push(*target);
                    }
                });
                Some(final_targets)
            }
        };
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
