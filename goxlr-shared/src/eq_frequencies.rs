use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Enum, Serialize, Deserialize)]
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

#[derive(Debug, Copy, Clone, Enum, Serialize, Deserialize)]
pub enum MiniFrequencies {
    Eq90h,
    Eq250h,
    Eq500h,
    Eq1kh,
    Eq3kh,
    Eq8kh,
}
