// TODO: This probably needs some tweaking..

use enum_map::Enum;
use enumset::{EnumSet, EnumSetType};
use goxlr_shared::interaction::InteractiveButtons;
use strum::EnumIter;

#[derive(Debug, Copy, Clone)]
pub enum ButtonStates {
    Colour1 = 0x01,
    Colour2 = 0x00,
    DimmedColour1 = 0x02,
    DimmedColour2 = 0x04,
    Flashing = 0x03,
}

#[derive(Debug, Copy, Clone)]
pub struct CurrentButtonStates {
    pub pressed: EnumSet<ButtonIndex>,
    pub volumes: [u8; 4],
    pub encoders: [i8; 4],
}

#[derive(EnumSetType, Enum, EnumIter, Debug)]
pub enum ButtonIndex {
    // These are all the buttons from the GoXLR Mini.
    Fader1Mute = 4,
    Fader2Mute = 9,
    Fader3Mute = 14,
    Fader4Mute = 19,
    Swear = 22,
    CoughButton = 23,

    // The rest are GoXLR Full Buttons. On the mini, they will simply be ignored.
    EffectSelect1 = 0,
    EffectSelect2 = 5,
    EffectSelect3 = 10,
    EffectSelect4 = 15,
    EffectSelect5 = 1,
    EffectSelect6 = 6,

    EffectFx = 21,
    EffectMegaphone = 20,
    EffectRobot = 11,
    EffectHardTune = 16,

    SamplerSelectA = 2,
    SamplerSelectB = 7,
    SamplerSelectC = 12,

    SamplerTopLeft = 3,
    SamplerTopRight = 8,
    SamplerBottomLeft = 17,
    SamplerBottomRight = 13,
    SamplerClear = 18,
}

impl Into<ButtonIndex> for InteractiveButtons {
    fn into(self) -> ButtonIndex {
        match self {
            InteractiveButtons::Fader1Mute => ButtonIndex::Fader1Mute,
            InteractiveButtons::Fader2Mute => ButtonIndex::Fader2Mute,
            InteractiveButtons::Fader3Mute => ButtonIndex::Fader3Mute,
            InteractiveButtons::Fader4Mute => ButtonIndex::Fader4Mute,
            InteractiveButtons::Swear => ButtonIndex::Swear,
            InteractiveButtons::CoughButton => ButtonIndex::CoughButton,
            InteractiveButtons::EffectSelect1 => ButtonIndex::EffectSelect1,
            InteractiveButtons::EffectSelect2 => ButtonIndex::EffectSelect2,
            InteractiveButtons::EffectSelect3 => ButtonIndex::EffectSelect3,
            InteractiveButtons::EffectSelect4 => ButtonIndex::EffectSelect4,
            InteractiveButtons::EffectSelect5 => ButtonIndex::EffectSelect5,
            InteractiveButtons::EffectSelect6 => ButtonIndex::EffectSelect6,
            InteractiveButtons::EffectFx => ButtonIndex::EffectFx,
            InteractiveButtons::EffectMegaphone => ButtonIndex::EffectMegaphone,
            InteractiveButtons::EffectRobot => ButtonIndex::EffectRobot,
            InteractiveButtons::EffectHardTune => ButtonIndex::EffectHardTune,
            InteractiveButtons::SamplerSelectA => ButtonIndex::SamplerSelectA,
            InteractiveButtons::SamplerSelectB => ButtonIndex::SamplerSelectB,
            InteractiveButtons::SamplerSelectC => ButtonIndex::SamplerSelectC,
            InteractiveButtons::SamplerTopLeft => ButtonIndex::SamplerTopLeft,
            InteractiveButtons::SamplerTopRight => ButtonIndex::SamplerTopRight,
            InteractiveButtons::SamplerBottomLeft => ButtonIndex::SamplerBottomLeft,
            InteractiveButtons::SamplerBottomRight => ButtonIndex::SamplerBottomRight,
            InteractiveButtons::SamplerClear => ButtonIndex::SamplerClear,
        }
    }
}

impl Into<InteractiveButtons> for ButtonIndex {
    fn into(self) -> InteractiveButtons {
        match self {
            ButtonIndex::Fader1Mute => InteractiveButtons::Fader1Mute,
            ButtonIndex::Fader2Mute => InteractiveButtons::Fader2Mute,
            ButtonIndex::Fader3Mute => InteractiveButtons::Fader3Mute,
            ButtonIndex::Fader4Mute => InteractiveButtons::Fader4Mute,
            ButtonIndex::Swear => InteractiveButtons::Swear,
            ButtonIndex::CoughButton => InteractiveButtons::CoughButton,
            ButtonIndex::EffectSelect1 => InteractiveButtons::EffectSelect1,
            ButtonIndex::EffectSelect2 => InteractiveButtons::EffectSelect2,
            ButtonIndex::EffectSelect3 => InteractiveButtons::EffectSelect3,
            ButtonIndex::EffectSelect4 => InteractiveButtons::EffectSelect4,
            ButtonIndex::EffectSelect5 => InteractiveButtons::EffectSelect5,
            ButtonIndex::EffectSelect6 => InteractiveButtons::EffectSelect6,
            ButtonIndex::EffectFx => InteractiveButtons::EffectFx,
            ButtonIndex::EffectMegaphone => InteractiveButtons::EffectMegaphone,
            ButtonIndex::EffectRobot => InteractiveButtons::EffectRobot,
            ButtonIndex::EffectHardTune => InteractiveButtons::EffectHardTune,
            ButtonIndex::SamplerSelectA => InteractiveButtons::SamplerSelectA,
            ButtonIndex::SamplerSelectB => InteractiveButtons::SamplerSelectB,
            ButtonIndex::SamplerSelectC => InteractiveButtons::SamplerSelectC,
            ButtonIndex::SamplerTopLeft => InteractiveButtons::SamplerTopLeft,
            ButtonIndex::SamplerTopRight => InteractiveButtons::SamplerTopRight,
            ButtonIndex::SamplerBottomLeft => InteractiveButtons::SamplerBottomLeft,
            ButtonIndex::SamplerBottomRight => InteractiveButtons::SamplerBottomRight,
            ButtonIndex::SamplerClear => InteractiveButtons::SamplerClear,
        }
    }
}
