#[cfg(feature = "clap")]
use clap::ValueEnum;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use enum_map::Enum;
use strum::EnumIter;

#[derive(Debug, Copy, Clone, Enum, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum Frequencies {
    Eq31h,
    Eq63h,
    Eq125h,
    Eq250h,
    Eq500h,
    Eq1kh,
    Eq2kh,
    Eq4kh,
    Eq8kh,
    Eq16kh,
}

#[derive(Debug, Copy, Clone, Enum, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum MiniFrequencies {
    Eq90h,
    Eq250h,
    Eq500h,
    Eq1kh,
    Eq3kh,
    Eq8kh,
}
