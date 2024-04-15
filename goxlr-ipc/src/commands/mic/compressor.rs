use goxlr_shared::compressor::{CompressorAttackTime, CompressorRatio, CompressorReleaseTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressorCommand {
    SetThreshold(i8),
    SetRatio(CompressorRatio),
    SetAttack(CompressorAttackTime),
    SetRelease(CompressorReleaseTime),
    SetMakeupGain(i8),
}
