use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum CompressorAttackTime {
    // Note: 0ms is technically 0.001ms
    Comp0ms,
    Comp2ms,
    Comp3ms,
    Comp4ms,
    Comp5ms,
    Comp6ms,
    Comp7ms,
    Comp8ms,
    Comp9ms,
    Comp10ms,
    Comp12ms,
    Comp14ms,
    Comp16ms,
    Comp18ms,
    Comp20ms,
    Comp23ms,
    Comp26ms,
    Comp30ms,
    Comp35ms,
    Comp40ms,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum CompressorReleaseTime {
    // Note: 0 is technically 15 :)
    Comp0ms,
    Comp15ms,
    Comp25ms,
    Comp35ms,
    Comp45ms,
    Comp55ms,
    Comp65ms,
    Comp75ms,
    Comp85ms,
    Comp100ms,
    Comp115ms,
    Comp140ms,
    Comp170ms,
    Comp230ms,
    Comp340ms,
    Comp680ms,
    Comp1000ms,
    Comp1500ms,
    Comp2000ms,
    Comp3000ms,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
