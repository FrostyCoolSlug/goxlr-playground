use anyhow::{bail, Result};
use enum_map::EnumMap;
use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::channels::input::InputChannels;
use goxlr_shared::channels::output::{OutputChannels, RoutingOutput};
use log::debug;
use strum::IntoEnumIterator;

use goxlr_shared::routing::RouteValue;
use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::device::GoXLR;

// These just help keep the function definitions slightly tidier...
type In = InputChannels;
type Out = RoutingOutput;
type Value = RouteValue;
type Row = EnumMap<RoutingOutput, RouteValue>;

/// Commands responsible for manipulating the Routing Table, these functions
/// will return 'true' if a change has actually occurred.
pub(crate) trait RoutingHandler {
    fn enable_route(&mut self, input: In, out: Out) -> Result<bool>;
    fn disable_route(&mut self, input: In, out: Out) -> Result<bool>;
    fn set_route_value(&mut self, input: In, out: Out, value: u8) -> Result<bool>;
    fn set_route(&mut self, input: In, out: Out, value: Value) -> Result<bool>;
    fn set_routing_row_from_profile(&mut self, input: In, values: EnumMap<OutputChannels, bool>);
    fn get_routing_input_row(&self, input: In) -> Row;

    // Commands for actually sending routing information to the GoXLR..
    async fn apply_routing_for_channel(&self, source: In) -> Result<()>;

    /// Global method for checking whether a target is valid for routing
    fn is_valid_routing_target(channel: FaderChannels) -> bool;
}

impl RoutingHandler for GoXLR {
    fn enable_route(&mut self, input: In, out: Out) -> Result<bool> {
        self.set_route(input, out, Value::On)
    }

    fn disable_route(&mut self, input: In, out: Out) -> Result<bool> {
        self.set_route(input, out, Value::Off)
    }

    fn set_route_value(&mut self, input: In, out: Out, value: u8) -> Result<bool> {
        if value > 32 {
            bail!("Value must be < 32, received: {}", value);
        }

        self.set_route(input, out, Value::Value(value))
    }

    fn set_route(&mut self, input: In, out: Out, value: Value) -> Result<bool> {
        // Just in case someone is bypassing set_route_value..
        if let Value::Value(value) = value {
            if value > 32 {
                bail!("Value must be < 32, received: {}", value);
            }
        }

        // If we're already set to this value, return and indicate no change
        if self.routing_state.get_routing(input, out) == value {
            return Ok(false);
        }

        // Set the Routing State to 'On'
        self.routing_state.set_routing(input, out, value);

        Ok(true)
    }

    fn set_routing_row_from_profile(&mut self, input: In, values: EnumMap<OutputChannels, bool>) {
        for output in OutputChannels::iter() {
            let route_out = RoutingOutput::from(output);
            let out_value = match values[output] {
                true => RouteValue::On,
                false => RouteValue::Off,
            };

            // This wont throw an error, we're not setting RouteValue::Value.
            let _ = self.set_route(input, route_out, out_value);
        }
    }

    fn get_routing_input_row(&self, input: In) -> Row {
        self.routing_state.get_input_routes(input)
    }

    async fn apply_routing_for_channel(&self, source: In) -> Result<()> {
        let routes = self.get_routing_input_row(source);

        debug!("Routing {:?} to {:?}", source, routes);

        let command = BasicResultCommand::ApplyRouting(source, routes);
        self.send_no_result(command).await
    }

    /// Headphone, LineOut and MicrophoneMonitor *ARE* valid mute targets, but they're
    /// not valid routing targets. This helper method allows code to check.
    fn is_valid_routing_target(channel: FaderChannels) -> bool {
        if channel == FaderChannels::Headphones {
            return false;
        }

        if channel == FaderChannels::LineOut {
            return false;
        }

        true
    }
}
