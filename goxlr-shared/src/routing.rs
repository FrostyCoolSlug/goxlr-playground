use enum_map::{Enum, EnumMap};

use crate::channels::{InputChannels, RoutingOutput};

struct RoutingTable {
    table: EnumMap<InputChannels, EnumMap<RoutingOutput, RouteValue>>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self {
            table: Default::default(),
        }
    }

    pub fn set_routing(&mut self, output: RoutingOutput, input: InputChannels, value: RouteValue) {
        // This format isn't supported, so do nothing.
        if output == RoutingOutput::ChatMic && input == InputChannels::Chat {
            return;
        }

        self.table[input][output] = value;
    }
}

#[derive(Debug, Default, Copy, Clone, Enum)]
pub enum RouteValue {
    On,
    #[default]
    Off,
    Value(u8),
}
