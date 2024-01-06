type SharedParams = goxlr_shared::microphone::MicrophoneParamKeys;
pub(crate) enum MicrophoneType {
    XLR = 0x01,
    Phantom = 0x02,
    TRS = 0x03,
}

impl MicrophoneType {
    pub fn get_gain_param(&self) -> SharedParams {
        match self {
            MicrophoneType::XLR => SharedParams::XLRGain,
            MicrophoneType::Phantom => SharedParams::PhantomGain,
            MicrophoneType::TRS => SharedParams::TRSGain,
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
            MicType::TRS => MicrophoneType::TRS,
        }
    }
}
