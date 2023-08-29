use goxlr_shared::states::State;

#[derive(Default)]
pub(crate) enum ButtonDisplay {
    Colour1 = 0x01,
    Colour2 = 0x00,

    #[default]
    DimmedColour1 = 0x02,
    DimmedColour2 = 0x04,
    Blinking = 0x03,
}

impl From<State> for ButtonDisplay {
    fn from(value: State) -> Self {
        match value {
            State::Colour1 => ButtonDisplay::Colour1,
            State::Colour2 => ButtonDisplay::Colour2,
            State::DimmedColour1 => ButtonDisplay::DimmedColour1,
            State::DimmedColour2 => ButtonDisplay::DimmedColour2,
            State::Blinking => ButtonDisplay::Blinking,
        }
    }
}
