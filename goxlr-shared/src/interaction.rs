/*
   The enums below are explicitly used in the event of some kind of detected interaction with the
   GoXLR. While they do match other types, it's best to implement .into() to remap, this allows us
   to ensure there are no misconceptions regarding the types and associations.
*/

use enum_map::{Enum, EnumMap};
use strum::EnumIter;

#[derive(Debug, Default)]
pub struct CurrentStates {
    pub buttons: EnumMap<InteractiveButtons, ButtonStates>,
    pub volumes: EnumMap<InteractiveFaders, u8>,
    pub encoders: EnumMap<InteractiveEncoders, i8>,
}

#[derive(Debug, Enum, EnumIter, Copy, Clone)]
pub enum InteractiveButtons {
    Fader1Mute,
    Fader2Mute,
    Fader3Mute,
    Fader4Mute,

    // Microphone Buttons
    Swear,
    CoughButton,

    // Effect Buttons
    EffectSelect1,
    EffectSelect2,
    EffectSelect3,
    EffectSelect4,
    EffectSelect5,
    EffectSelect6,

    EffectFx,
    EffectMegaphone,
    EffectRobot,
    EffectHardTune,

    // Sampler Buttons
    SamplerSelectA,
    SamplerSelectB,
    SamplerSelectC,

    SamplerTopLeft,
    SamplerTopRight,
    SamplerBottomLeft,
    SamplerBottomRight,
    SamplerClear,
}

#[derive(Debug, Enum, EnumIter, Copy, Clone)]
pub enum InteractiveFaders {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Enum, EnumIter, Copy, Clone)]
pub enum InteractiveEncoders {
    Pitch,
    Gender,
    Reverb,
    Echo,
}

#[derive(Debug, Default, Enum, Copy, Clone, Eq, PartialEq)]
pub enum ButtonStates {
    #[default]
    NotPressed,
    Pressed,
}
