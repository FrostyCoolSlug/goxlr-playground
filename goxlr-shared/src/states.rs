use enum_map::EnumMap;

use crate::buttons::{Buttons, InactiveButtonBehaviour};

#[derive(Debug, Default, Copy, Clone)]
pub struct ButtonDisplayStates {
    states: EnumMap<Buttons, State>,
}

impl ButtonDisplayStates {
    pub fn set_state(&mut self, button: Buttons, state: State) {
        self.states[button] = state;
    }
    pub fn get_state(&self, button: Buttons) -> State {
        self.states[button]
    }
    pub fn get_list(&self) -> EnumMap<Buttons, State> {
        self.states
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub enum State {
    Colour1,
    Colour2,
    #[default]
    DimmedColour1,
    DimmedColour2,
    Blinking,
}

impl From<InactiveButtonBehaviour> for State {
    fn from(value: InactiveButtonBehaviour) -> Self {
        match value {
            InactiveButtonBehaviour::DimActive => State::DimmedColour1,
            InactiveButtonBehaviour::DimInactive => State::DimmedColour2,
            InactiveButtonBehaviour::InactiveColour => State::Colour2,
        }
    }
}
