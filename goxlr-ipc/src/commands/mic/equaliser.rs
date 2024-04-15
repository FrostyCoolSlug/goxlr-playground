use goxlr_shared::eq_frequencies::{Frequencies, MiniFrequencies};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EqualiserCommand {
    Mini(MiniEqualiserCommand),
    Full(FullEqualiserCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MiniEqualiserCommand {
    SetFrequency(SetMiniFrequency),
    SetGain(SetMiniGain),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetMiniFrequency {
    pub base: MiniFrequencies,
    pub frequency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetMiniGain {
    pub base: MiniFrequencies,
    pub gain: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FullEqualiserCommand {
    SetFrequency(SetFullFrequency),
    SetGain(SetFullGain),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFullFrequency {
    pub base: Frequencies,
    pub frequency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFullGain {
    pub base: Frequencies,
    pub gain: i8,
}
