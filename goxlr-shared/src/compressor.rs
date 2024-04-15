#[cfg(feature = "clap")]
use clap::ValueEnum;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum CompressorAttackTime {
    // Note: 0ms is technically 0.001ms
    Attack0ms,
    Attack2ms,
    Attack3ms,
    Attack4ms,
    Attack5ms,
    Attack6ms,
    Attack7ms,
    Attack8ms,
    Attack9ms,
    Attack10ms,
    Attack12ms,
    Attack14ms,
    Attack16ms,
    Attack18ms,
    Attack20ms,
    Attack23ms,
    Attack26ms,
    Attack30ms,
    Attack35ms,
    Attack40ms,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum CompressorReleaseTime {
    // Note: 0 is technically 15 :)
    Release0ms,
    Release15ms,
    Release25ms,
    Release35ms,
    Release45ms,
    Release55ms,
    Release65ms,
    Release75ms,
    Release85ms,
    Release100ms,
    Release115ms,
    Release140ms,
    Release170ms,
    Release230ms,
    Release340ms,
    Release680ms,
    Release1000ms,
    Release1500ms,
    Release2000ms,
    Release3000ms,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum CompressorRatio {
    Ratio1_0,
    Ratio1_1,
    Ratio1_2,
    Ratio1_4,
    Ratio1_6,
    Ratio1_8,
    Ratio2_0,
    Ratio2_5,
    Ratio3_2,
    Ratio4_0,
    Ratio5_6,
    Ratio8_0,
    Ratio16_0,
    Ratio32_0,
    Ratio64_0,
}
