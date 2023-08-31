use enum_map::Enum;
use goxlr_shared::faders::Fader;
use goxlr_shared::interaction::InteractiveFaders;
use strum::EnumIter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum, EnumIter)]
pub(crate) enum DeviceFader {
    A = 0x00,
    B = 0x01,
    C = 0x02,
    D = 0x03,
}

impl From<Fader> for DeviceFader {
    fn from(value: Fader) -> Self {
        match value {
            Fader::A => DeviceFader::A,
            Fader::B => DeviceFader::B,
            Fader::C => DeviceFader::C,
            Fader::D => DeviceFader::D,
        }
    }
}

impl From<InteractiveFaders> for DeviceFader {
    fn from(value: InteractiveFaders) -> Self {
        match value {
            InteractiveFaders::A => DeviceFader::A,
            InteractiveFaders::B => DeviceFader::B,
            InteractiveFaders::C => DeviceFader::C,
            InteractiveFaders::D => DeviceFader::D,
        }
    }
}

impl From<DeviceFader> for InteractiveFaders {
    fn from(value: DeviceFader) -> Self {
        match value {
            DeviceFader::A => InteractiveFaders::A,
            DeviceFader::B => InteractiveFaders::B,
            DeviceFader::C => InteractiveFaders::C,
            DeviceFader::D => InteractiveFaders::D,
        }
    }
}
