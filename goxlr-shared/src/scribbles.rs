use crate::faders::Fader;

pub enum Scribble {
    A,
    B,
    C,
    D,
}

impl From<Fader> for Scribble {
    fn from(value: Fader) -> Self {
        match value {
            Fader::A => Scribble::A,
            Fader::B => Scribble::B,
            Fader::C => Scribble::C,
            Fader::D => Scribble::D,
        }
    }
}
