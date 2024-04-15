use crate::eq_frequencies::{Frequencies, MiniFrequencies};
use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Enum, Serialize, Deserialize)]
pub enum MicrophoneType {
    XLR,
    Phantom,
    Jack,
}

/*
 As with everything else, we're going to keep the values to these keys isolated in the USB crate
 and have alternatives for direct access in goxlr-shared, where we don't have to care about
 changes
*/

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum MicParamKeys {
    GateThreshold,
    GateAttack,
    GateRelease,
    GateAttenuation,

    CompressorThreshold,
    CompressorRatio,
    CompressorAttack,
    CompressorRelease,
    CompressorMakeUpGain,

    BleepLevel,

    /*
     These are the values for the GoXLR mini, it seems there's a difference in how the two
     are setup, The Mini does EQ via microphone parameters, where as the full does it via effects.
    */
    Equalizer90HzFrequency,
    Equalizer90HzGain,
    Equalizer250HzFrequency,
    Equalizer250HzGain,
    Equalizer500HzFrequency,
    Equalizer500HzGain,
    Equalizer1KHzFrequency,
    Equalizer1KHzGain,
    Equalizer3KHzFrequency,
    Equalizer3KHzGain,
    Equalizer8KHzFrequency,
    Equalizer8KHzGain,
}

impl MicParamKeys {
    pub fn from_eq_gain(value: MiniFrequencies) -> Self {
        match value {
            MiniFrequencies::Eq90h => MicParamKeys::Equalizer90HzGain,
            MiniFrequencies::Eq250h => MicParamKeys::Equalizer250HzGain,
            MiniFrequencies::Eq500h => MicParamKeys::Equalizer500HzGain,
            MiniFrequencies::Eq1kh => MicParamKeys::Equalizer1KHzGain,
            MiniFrequencies::Eq3kh => MicParamKeys::Equalizer3KHzGain,
            MiniFrequencies::Eq8kh => MicParamKeys::Equalizer8KHzGain,
        }
    }

    pub fn from_eq_freq(value: MiniFrequencies) -> Self {
        match value {
            MiniFrequencies::Eq90h => MicParamKeys::Equalizer90HzFrequency,
            MiniFrequencies::Eq250h => MicParamKeys::Equalizer250HzFrequency,
            MiniFrequencies::Eq500h => MicParamKeys::Equalizer500HzFrequency,
            MiniFrequencies::Eq1kh => MicParamKeys::Equalizer1KHzFrequency,
            MiniFrequencies::Eq3kh => MicParamKeys::Equalizer3KHzFrequency,
            MiniFrequencies::Eq8kh => MicParamKeys::Equalizer8KHzFrequency,
        }
    }
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum MicEffectKeys {
    MicInputMute,
    BleepLevel,
    DeEsser,

    GateMode,
    GateThreshold,
    GateEnabled,
    GateAttenuation,
    GateAttack,
    GateRelease,

    MicCompSelect,
    CompressorRatio,
    CompressorAttack,
    CompressorRelease,
    CompressorMakeUpGain,

    Equalizer31HzFrequency,
    Equalizer31HzGain,
    Equalizer63HzFrequency,
    Equalizer63HzGain,
    Equalizer125HzFrequency,
    Equalizer125HzGain,
    Equalizer250HzFrequency,
    Equalizer250HzGain,
    Equalizer500HzFrequency,
    Equalizer500HzGain,
    Equalizer1KHzFrequency,
    Equalizer1KHzGain,
    Equalizer2KHzFrequency,
    Equalizer2KHzGain,
    Equalizer4KHzFrequency,
    Equalizer4KHzGain,
    Equalizer8KHzFrequency,
    Equalizer8KHzGain,
    Equalizer16KHzFrequency,
    Equalizer16KHzGain,
    CompressorThreshold,

    ReverbAmount,
    ReverbDecay,
    ReverbEarlyLevel,
    ReverbTailLevel, // Always sent as 0.
    ReverbPredelay,
    ReverbLowColor,
    ReverbHighColor,
    ReverbHighFactor,
    ReverbDiffuse,
    ReverbModSpeed,
    ReverbModDepth,
    ReverbType,

    EchoAmount,
    EchoFeedback,
    EchoTempo,
    EchoDelayL,
    EchoDelayR,
    EchoFeedbackL,
    EchoFeedbackR,
    EchoXFBLtoR,
    EchoXFBRtoL,
    EchoSource,
    EchoDivL,
    EchoDivR,
    EchoFilterStyle,

    PitchAmount,
    PitchCharacter,
    PitchThreshold,

    GenderAmount,

    MegaphoneAmount,
    MegaphonePostGain,
    MegaphoneStyle,
    MegaphoneHP,
    MegaphoneLP,
    MegaphonePreGain,
    MegaphoneDistType,
    MegaphonePresenceGain,
    MegaphonePresenceFC,
    MegaphonePresenceBW,
    MegaphoneBeatboxEnable,
    MegaphoneFilterControl,
    MegaphoneFilter,
    MegaphoneDrivePotGainCompMid,
    MegaphoneDrivePotGainCompMax,

    RobotLowGain,
    RobotLowFreq,
    RobotLowWidth,
    RobotMidGain,
    RobotMidFreq,
    RobotMidWidth,
    RobotHiGain,
    RobotHiFreq,
    RobotHiWidth,
    RobotWaveform,
    RobotPulseWidth,
    RobotThreshold,
    RobotDryMix,
    RobotStyle,

    HardTuneKeySource, // Always sent as 0.
    HardTuneAmount,
    HardTuneRate,
    HardTuneWindow,
    HardTuneScale,
    HardTunePitchAmount,

    RobotEnabled,
    MegaphoneEnabled,
    HardTuneEnabled,

    Encoder1Enabled,
    Encoder2Enabled,
    Encoder3Enabled,
    Encoder4Enabled,
}

impl MicEffectKeys {
    pub fn from_eq_gain(value: Frequencies) -> Self {
        match value {
            Frequencies::Eq31h => MicEffectKeys::Equalizer31HzGain,
            Frequencies::Eq63h => MicEffectKeys::Equalizer63HzGain,
            Frequencies::Eq125h => MicEffectKeys::Equalizer125HzGain,
            Frequencies::Eq250h => MicEffectKeys::Equalizer250HzGain,
            Frequencies::Eq500h => MicEffectKeys::Equalizer500HzGain,
            Frequencies::Eq1kh => MicEffectKeys::Equalizer1KHzGain,
            Frequencies::Eq2kh => MicEffectKeys::Equalizer2KHzGain,
            Frequencies::Eq4kh => MicEffectKeys::Equalizer4KHzGain,
            Frequencies::Eq8kh => MicEffectKeys::Equalizer8KHzGain,
            Frequencies::Eq16kh => MicEffectKeys::Equalizer16KHzGain,
        }
    }

    pub fn from_eq_freq(value: Frequencies) -> Self {
        match value {
            Frequencies::Eq31h => MicEffectKeys::Equalizer31HzFrequency,
            Frequencies::Eq63h => MicEffectKeys::Equalizer63HzFrequency,
            Frequencies::Eq125h => MicEffectKeys::Equalizer125HzFrequency,
            Frequencies::Eq250h => MicEffectKeys::Equalizer250HzFrequency,
            Frequencies::Eq500h => MicEffectKeys::Equalizer500HzFrequency,
            Frequencies::Eq1kh => MicEffectKeys::Equalizer1KHzFrequency,
            Frequencies::Eq2kh => MicEffectKeys::Equalizer2KHzFrequency,
            Frequencies::Eq4kh => MicEffectKeys::Equalizer4KHzFrequency,
            Frequencies::Eq8kh => MicEffectKeys::Equalizer8KHzFrequency,
            Frequencies::Eq16kh => MicEffectKeys::Equalizer16KHzFrequency,
        }
    }
}
