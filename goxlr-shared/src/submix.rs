// This is an oddly empty file, we don't need anything more here because we can use InputChannel
// to point to the input needed, and the USB crate will take care of it!
use enum_map::Enum;
use strum::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Enum, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Mix {
    #[default]
    A,
    B,
}
