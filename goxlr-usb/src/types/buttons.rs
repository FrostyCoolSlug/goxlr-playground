use enum_map::Enum;
use enumset::{EnumSet, EnumSetType};
use goxlr_shared::buttons::Buttons;
use goxlr_shared::interaction::InteractiveButtons;
use strum::EnumIter;

#[derive(Debug, Copy, Clone)]
pub(crate) struct CurrentButtonStates {
    pub pressed: EnumSet<PhysicalButton>,
    pub volumes: [u8; 4],
    pub encoders: [i8; 4],
}

#[derive(EnumSetType, Enum, EnumIter, Debug)]
pub(crate) enum PhysicalButton {
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

impl From<InteractiveButtons> for PhysicalButton {
    fn from(value: InteractiveButtons) -> Self {
        match value {
            InteractiveButtons::Fader1Mute => PhysicalButton::Fader1Mute,
            InteractiveButtons::Fader2Mute => PhysicalButton::Fader2Mute,
            InteractiveButtons::Fader3Mute => PhysicalButton::Fader3Mute,
            InteractiveButtons::Fader4Mute => PhysicalButton::Fader4Mute,
            InteractiveButtons::Swear => PhysicalButton::Swear,
            InteractiveButtons::CoughButton => PhysicalButton::CoughButton,
            InteractiveButtons::EffectSelect1 => PhysicalButton::EffectSelect1,
            InteractiveButtons::EffectSelect2 => PhysicalButton::EffectSelect2,
            InteractiveButtons::EffectSelect3 => PhysicalButton::EffectSelect3,
            InteractiveButtons::EffectSelect4 => PhysicalButton::EffectSelect4,
            InteractiveButtons::EffectSelect5 => PhysicalButton::EffectSelect5,
            InteractiveButtons::EffectSelect6 => PhysicalButton::EffectSelect6,
            InteractiveButtons::EffectFx => PhysicalButton::EffectFx,
            InteractiveButtons::EffectMegaphone => PhysicalButton::EffectMegaphone,
            InteractiveButtons::EffectRobot => PhysicalButton::EffectRobot,
            InteractiveButtons::EffectHardTune => PhysicalButton::EffectHardTune,
            InteractiveButtons::SamplerSelectA => PhysicalButton::SamplerSelectA,
            InteractiveButtons::SamplerSelectB => PhysicalButton::SamplerSelectB,
            InteractiveButtons::SamplerSelectC => PhysicalButton::SamplerSelectC,
            InteractiveButtons::SamplerTopLeft => PhysicalButton::SamplerTopLeft,
            InteractiveButtons::SamplerTopRight => PhysicalButton::SamplerTopRight,
            InteractiveButtons::SamplerBottomLeft => PhysicalButton::SamplerBottomLeft,
            InteractiveButtons::SamplerBottomRight => PhysicalButton::SamplerBottomRight,
            InteractiveButtons::SamplerClear => PhysicalButton::SamplerClear,
        }
    }
}

impl From<Buttons> for PhysicalButton {
    fn from(value: Buttons) -> Self {
        match value {
            Buttons::FaderA => PhysicalButton::Fader1Mute,
            Buttons::FaderB => PhysicalButton::Fader2Mute,
            Buttons::FaderC => PhysicalButton::Fader3Mute,
            Buttons::FaderD => PhysicalButton::Fader4Mute,
            Buttons::Swear => PhysicalButton::Swear,
            Buttons::CoughButton => PhysicalButton::CoughButton,
            Buttons::EffectSelect1 => PhysicalButton::EffectSelect1,
            Buttons::EffectSelect2 => PhysicalButton::EffectSelect2,
            Buttons::EffectSelect3 => PhysicalButton::EffectSelect3,
            Buttons::EffectSelect4 => PhysicalButton::EffectSelect4,
            Buttons::EffectSelect5 => PhysicalButton::EffectSelect5,
            Buttons::EffectSelect6 => PhysicalButton::EffectSelect6,
            Buttons::EffectFx => PhysicalButton::EffectFx,
            Buttons::EffectMegaphone => PhysicalButton::EffectMegaphone,
            Buttons::EffectRobot => PhysicalButton::EffectRobot,
            Buttons::EffectHardTune => PhysicalButton::EffectHardTune,
            Buttons::SamplerSelectA => PhysicalButton::SamplerSelectA,
            Buttons::SamplerSelectB => PhysicalButton::SamplerSelectB,
            Buttons::SamplerSelectC => PhysicalButton::SamplerSelectC,
            Buttons::SamplerTopLeft => PhysicalButton::SamplerTopLeft,
            Buttons::SamplerTopRight => PhysicalButton::SamplerTopRight,
            Buttons::SamplerBottomLeft => PhysicalButton::SamplerBottomLeft,
            Buttons::SamplerBottomRight => PhysicalButton::SamplerBottomRight,
            Buttons::SamplerClear => PhysicalButton::SamplerClear,
        }
    }
}
