pub mod compressor;
pub mod equaliser;
pub mod gate;
pub mod setup;

use crate::commands::mic::compressor::CompressorCommand;
use crate::commands::mic::equaliser::EqualiserCommand;
use crate::commands::mic::gate::GateCommand;
use crate::commands::mic::setup::SetupCommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MicrophoneCommand {
    Setup(SetupCommand),
    Equaliser(EqualiserCommand),
    Compressor(CompressorCommand),
    Gate(GateCommand),
    GetMicLevel,
}
