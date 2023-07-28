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
