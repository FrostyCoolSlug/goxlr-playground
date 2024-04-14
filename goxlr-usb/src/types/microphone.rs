use crate::types::mic_keys::MicParamKeys;

type SharedParams = goxlr_shared::microphone::MicParamKeys;
pub(crate) enum MicrophoneType {
    XLR = 0x01,
    Phantom = 0x02,
    Jack = 0x03,
}

impl MicrophoneType {
    pub fn get_gain_param(&self) -> MicParamKeys {
        match self {
            MicrophoneType::XLR => MicParamKeys::XLRGain,
            MicrophoneType::Phantom => MicParamKeys::PhantomGain,
            MicrophoneType::Jack => MicParamKeys::JackGain,
        }
    }

    pub fn has_phantom(&self) -> bool {
        matches!(self, MicrophoneType::Phantom)
    }
}

type MicType = goxlr_shared::microphone::MicrophoneType;

impl From<MicType> for MicrophoneType {
    fn from(value: MicType) -> Self {
        match value {
            MicType::XLR => MicrophoneType::XLR,
            MicType::Phantom => MicrophoneType::Phantom,
            MicType::Jack => MicrophoneType::Jack,
        }
    }
}
