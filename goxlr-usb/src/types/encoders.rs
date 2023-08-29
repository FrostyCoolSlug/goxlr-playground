use goxlr_shared::encoders::Encoders;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeviceEncoder {
    Pitch = 0x00,
    Gender = 0x01,
    Reverb = 0x02,
    Echo = 0x03,
}

impl From<Encoders> for DeviceEncoder {
    fn from(value: Encoders) -> Self {
        match value {
            Encoders::Pitch => DeviceEncoder::Pitch,
            Encoders::Gender => DeviceEncoder::Gender,
            Encoders::Reverb => DeviceEncoder::Reverb,
            Encoders::Echo => DeviceEncoder::Echo,
        }
    }
}
