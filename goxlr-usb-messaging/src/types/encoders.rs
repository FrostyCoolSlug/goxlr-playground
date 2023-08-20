use goxlr_shared::encoders::Encoders;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Encoder {
    Pitch = 0x00,
    Gender = 0x01,
    Reverb = 0x02,
    Echo = 0x03,
}

impl Into<Encoder> for Encoders {
    fn into(self) -> Encoder {
        match self {
            Encoders::Pitch => Encoder::Pitch,
            Encoders::Gender => Encoder::Gender,
            Encoders::Reverb => Encoder::Reverb,
            Encoders::Echo => Encoder::Echo,
        }
    }
}
