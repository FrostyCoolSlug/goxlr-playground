use crate::interaction::InteractiveEncoders;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A simple list of the 4 encoders
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Encoders {
    Pitch,
    Gender,
    Reverb,
    Echo,
}

impl From<InteractiveEncoders> for Encoders {
    fn from(value: InteractiveEncoders) -> Self {
        match value {
            InteractiveEncoders::Pitch => Encoders::Pitch,
            InteractiveEncoders::Gender => Encoders::Gender,
            InteractiveEncoders::Reverb => Encoders::Reverb,
            InteractiveEncoders::Echo => Encoders::Echo,
        }
    }
}
