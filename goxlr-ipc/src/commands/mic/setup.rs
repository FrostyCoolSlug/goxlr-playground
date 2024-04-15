use goxlr_shared::microphone::MicrophoneType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetupCommand {
    SetMicType(MicrophoneType),
    SetMicGain(u8),
}
