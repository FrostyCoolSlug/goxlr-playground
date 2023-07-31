// TODO: This probably needs some tweaking..

use enum_map::Enum;
use enumset::{EnumSet, EnumSetType};
use goxlr_shared::interaction::InteractiveButtons;
use strum::EnumIter;

#[derive(Debug, Copy, Clone)]
pub struct CurrentButtonStates {
    pub pressed: EnumSet<StatusButton>,
    pub volumes: [u8; 4],
    pub encoders: [i8; 4],
}

#[derive(EnumSetType, Enum, EnumIter, Debug)]
pub enum StatusButton {
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

impl From<InteractiveButtons> for StatusButton {
    fn from(value: InteractiveButtons) -> Self {
        match value {
            InteractiveButtons::Fader1Mute => StatusButton::Fader1Mute,
            InteractiveButtons::Fader2Mute => StatusButton::Fader2Mute,
            InteractiveButtons::Fader3Mute => StatusButton::Fader3Mute,
            InteractiveButtons::Fader4Mute => StatusButton::Fader4Mute,
            InteractiveButtons::Swear => StatusButton::Swear,
            InteractiveButtons::CoughButton => StatusButton::CoughButton,
            InteractiveButtons::EffectSelect1 => StatusButton::EffectSelect1,
            InteractiveButtons::EffectSelect2 => StatusButton::EffectSelect2,
            InteractiveButtons::EffectSelect3 => StatusButton::EffectSelect3,
            InteractiveButtons::EffectSelect4 => StatusButton::EffectSelect4,
            InteractiveButtons::EffectSelect5 => StatusButton::EffectSelect5,
            InteractiveButtons::EffectSelect6 => StatusButton::EffectSelect6,
            InteractiveButtons::EffectFx => StatusButton::EffectFx,
            InteractiveButtons::EffectMegaphone => StatusButton::EffectMegaphone,
            InteractiveButtons::EffectRobot => StatusButton::EffectRobot,
            InteractiveButtons::EffectHardTune => StatusButton::EffectHardTune,
            InteractiveButtons::SamplerSelectA => StatusButton::SamplerSelectA,
            InteractiveButtons::SamplerSelectB => StatusButton::SamplerSelectB,
            InteractiveButtons::SamplerSelectC => StatusButton::SamplerSelectC,
            InteractiveButtons::SamplerTopLeft => StatusButton::SamplerTopLeft,
            InteractiveButtons::SamplerTopRight => StatusButton::SamplerTopRight,
            InteractiveButtons::SamplerBottomLeft => StatusButton::SamplerBottomLeft,
            InteractiveButtons::SamplerBottomRight => StatusButton::SamplerBottomRight,
            InteractiveButtons::SamplerClear => StatusButton::SamplerClear,
        }
    }
}
//
// impl From<StatusButton> for InteractiveButtons {
//     fn from(value: StatusButton) -> Self {
//         match value {
//             StatusButton::Fader1Mute => InteractiveButtons::Fader1Mute,
//             StatusButton::Fader2Mute => InteractiveButtons::Fader2Mute,
//             StatusButton::Fader3Mute => InteractiveButtons::Fader3Mute,
//             StatusButton::Fader4Mute => InteractiveButtons::Fader4Mute,
//             StatusButton::Swear => InteractiveButtons::Swear,
//             StatusButton::CoughButton => InteractiveButtons::CoughButton,
//             StatusButton::EffectSelect1 => InteractiveButtons::EffectSelect1,
//             StatusButton::EffectSelect2 => InteractiveButtons::EffectSelect2,
//             StatusButton::EffectSelect3 => InteractiveButtons::EffectSelect3,
//             StatusButton::EffectSelect4 => InteractiveButtons::EffectSelect4,
//             StatusButton::EffectSelect5 => InteractiveButtons::EffectSelect5,
//             StatusButton::EffectSelect6 => InteractiveButtons::EffectSelect6,
//             StatusButton::EffectFx => InteractiveButtons::EffectFx,
//             StatusButton::EffectMegaphone => InteractiveButtons::EffectMegaphone,
//             StatusButton::EffectRobot => InteractiveButtons::EffectRobot,
//             StatusButton::EffectHardTune => InteractiveButtons::EffectHardTune,
//             StatusButton::SamplerSelectA => InteractiveButtons::SamplerSelectA,
//             StatusButton::SamplerSelectB => InteractiveButtons::SamplerSelectB,
//             StatusButton::SamplerSelectC => InteractiveButtons::SamplerSelectC,
//             StatusButton::SamplerTopLeft => InteractiveButtons::SamplerTopLeft,
//             StatusButton::SamplerTopRight => InteractiveButtons::SamplerTopRight,
//             StatusButton::SamplerBottomLeft => InteractiveButtons::SamplerBottomLeft,
//             StatusButton::SamplerBottomRight => InteractiveButtons::SamplerBottomRight,
//             StatusButton::SamplerClear => InteractiveButtons::SamplerClear,
//         }
//     }
// }
