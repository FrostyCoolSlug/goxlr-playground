use goxlr_shared::gate::GateTimes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GateCommand {
    SetEnabled(bool),
    SetThreshold(i8),
    SetAttack(GateTimes),
    SetRelease(GateTimes),
    SetAttenuation(u8),
}
