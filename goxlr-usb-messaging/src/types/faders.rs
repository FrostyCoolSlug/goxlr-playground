use goxlr_shared::faders::Fader;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
