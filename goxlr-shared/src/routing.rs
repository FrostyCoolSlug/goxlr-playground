use enum_map::{Enum, EnumMap};

use crate::channels::{InputChannels, RoutingOutput};

// Types to help keep things tidy..
type Row = EnumMap<RoutingOutput, RouteValue>;
type Table = EnumMap<InputChannels, Row>;

#[derive(Default)]
pub struct RoutingTable {
    table: Table,
}

impl RoutingTable {
    pub fn set_routing(&mut self, input: InputChannels, output: RoutingOutput, value: RouteValue) {
        // This format isn't supported, so do nothing.
        if output == RoutingOutput::ChatMic && input == InputChannels::Chat {
            return;
        }

        self.table[input][output] = value;
    }

    pub fn get_routing(&self, input: InputChannels, output: RoutingOutput) -> RouteValue {
        self.table[input][output]
    }

    pub fn get_input_routes(&self, input: InputChannels) -> Row {
        self.table[input]
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Enum)]
pub enum RouteValue {
    On,
    #[default]
    Off,
    Value(u8),
}
