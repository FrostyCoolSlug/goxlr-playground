use anyhow::{bail, Result};
use enum_map::EnumMap;

use crate::device::goxlr::device::GoXLR;
use goxlr_shared::channels::{InputChannels, OutputChannels, RoutingOutput};
use goxlr_shared::faders::FaderSources;
use goxlr_shared::routing::RouteValue;

// These just help keep the function definitions slightly tidier..
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
    fn get_input_row(&mut self, input: In) -> Row;

    fn is_valid_routing_target(channel: FaderSources) -> bool;
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

    fn get_input_row(&mut self, input: In) -> Row {
        self.routing_state.get_input_routes(input)
    }

    /// Headphone, LineOut and MicrophoneMonitor *ARE* valid mute targets, but they're
    /// not valid routing targets. This helper method allows code to check.
    fn is_valid_routing_target(channel: FaderSources) -> bool {
        if channel == FaderSources::Headphones {
            return false;
        }

        if channel == FaderSources::LineOut {
            return false;
        }

        if channel == FaderSources::MicrophoneMonitor {
            return false;
        }

        return true;
    }
}
