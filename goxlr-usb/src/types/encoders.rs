use enum_map::Enum;
use goxlr_shared::encoders::Encoders;
use goxlr_shared::interaction::InteractiveEncoders;
use strum::EnumIter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum, EnumIter)]
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

impl From<InteractiveEncoders> for DeviceEncoder {
    fn from(value: InteractiveEncoders) -> Self {
        match value {
            InteractiveEncoders::Pitch => DeviceEncoder::Pitch,
            InteractiveEncoders::Gender => DeviceEncoder::Gender,
            InteractiveEncoders::Reverb => DeviceEncoder::Reverb,
            InteractiveEncoders::Echo => DeviceEncoder::Echo,
        }
    }
}

impl From<DeviceEncoder> for InteractiveEncoders {
    fn from(value: DeviceEncoder) -> Self {
        match value {
            DeviceEncoder::Pitch => InteractiveEncoders::Pitch,
            DeviceEncoder::Gender => InteractiveEncoders::Gender,
            DeviceEncoder::Reverb => InteractiveEncoders::Reverb,
            DeviceEncoder::Echo => InteractiveEncoders::Echo,
        }
    }
}
