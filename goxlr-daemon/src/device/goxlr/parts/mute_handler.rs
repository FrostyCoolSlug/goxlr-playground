use anyhow::{bail, Result};
use async_trait::async_trait;
use log::debug;
use strum::IntoEnumIterator;

use goxlr_shared::channels::ChannelMuteState::{Muted, Unmuted};
use goxlr_shared::channels::{ChannelMuteState, InputChannels, OutputChannels, RoutingOutput};
use goxlr_shared::faders::FaderSources;
use goxlr_shared::routing::RouteValue;
use goxlr_usb_messaging::events::commands::{BasicResultCommand, ChannelSource};

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::parts::routing_handler::RoutingHandler;

type Source = FaderSources;
type Target = Vec<OutputChannels>;

#[async_trait]
pub(crate) trait MuteHandler {
    async fn mute_to_target(&mut self, source: Source, targets: Target) -> Result<MuteChanges>;
    async fn mute_to_all(&mut self, source: Source) -> Result<MuteChanges>;
    async fn unmute(&mut self, source: Source) -> Result<MuteChanges>;
    async fn send_mute_state(&mut self, source: Source, state: ChannelMuteState) -> Result<()>;
    async fn update_mute_button_state(&mut self, source: Source) -> Result<bool>;
}

#[async_trait]
impl MuteHandler for GoXLR {
    async fn mute_to_target(&mut self, source: Source, targets: Target) -> Result<MuteChanges> {
        // If our target list is empty, activate 'mute to all' behaviour..
        if targets.is_empty() {
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
            vec![InputChannels::from(source)]
        } else {
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

    async fn update_mute_button_state(&mut self, source: Source) -> Result<bool> {
        Ok(false)
    }
}

/// This structure provides a list of things which have been changed by the mute commands,
/// generally speaking, they'll be followed up by applying them!
#[derive(Default)]
pub(crate) struct MuteChanges {
    routing: Vec<InputChannels>,
}
