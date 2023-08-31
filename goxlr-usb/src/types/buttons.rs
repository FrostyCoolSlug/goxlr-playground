use enum_map::Enum;
use enumset::{EnumSet, EnumSetType};
use goxlr_shared::buttons::Buttons;
use goxlr_shared::interaction::InteractiveButtons;
use strum::EnumIter;

#[derive(Debug, Copy, Clone)]
pub(crate) struct CurrentButtonStates {
    pub pressed: EnumSet<DeviceButton>,
    pub volumes: [u8; 4],
    pub encoders: [i8; 4],
}

#[derive(EnumSetType, Enum, EnumIter, Debug)]
pub(crate) enum DeviceButton {
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

impl From<InteractiveButtons> for DeviceButton {
    fn from(value: InteractiveButtons) -> Self {
        match value {
            InteractiveButtons::Fader1Mute => DeviceButton::Fader1Mute,
            InteractiveButtons::Fader2Mute => DeviceButton::Fader2Mute,
            InteractiveButtons::Fader3Mute => DeviceButton::Fader3Mute,
            InteractiveButtons::Fader4Mute => DeviceButton::Fader4Mute,
            InteractiveButtons::Swear => DeviceButton::Swear,
            InteractiveButtons::CoughButton => DeviceButton::CoughButton,
            InteractiveButtons::EffectSelect1 => DeviceButton::EffectSelect1,
            InteractiveButtons::EffectSelect2 => DeviceButton::EffectSelect2,
            InteractiveButtons::EffectSelect3 => DeviceButton::EffectSelect3,
            InteractiveButtons::EffectSelect4 => DeviceButton::EffectSelect4,
            InteractiveButtons::EffectSelect5 => DeviceButton::EffectSelect5,
            InteractiveButtons::EffectSelect6 => DeviceButton::EffectSelect6,
            InteractiveButtons::EffectFx => DeviceButton::EffectFx,
            InteractiveButtons::EffectMegaphone => DeviceButton::EffectMegaphone,
            InteractiveButtons::EffectRobot => DeviceButton::EffectRobot,
            InteractiveButtons::EffectHardTune => DeviceButton::EffectHardTune,
            InteractiveButtons::SamplerSelectA => DeviceButton::SamplerSelectA,
            InteractiveButtons::SamplerSelectB => DeviceButton::SamplerSelectB,
            InteractiveButtons::SamplerSelectC => DeviceButton::SamplerSelectC,
            InteractiveButtons::SamplerTopLeft => DeviceButton::SamplerTopLeft,
            InteractiveButtons::SamplerTopRight => DeviceButton::SamplerTopRight,
            InteractiveButtons::SamplerBottomLeft => DeviceButton::SamplerBottomLeft,
            InteractiveButtons::SamplerBottomRight => DeviceButton::SamplerBottomRight,
            InteractiveButtons::SamplerClear => DeviceButton::SamplerClear,
        }
    }
}

impl From<DeviceButton> for InteractiveButtons {
    fn from(value: DeviceButton) -> Self {
        match value {
            DeviceButton::Fader1Mute => InteractiveButtons::Fader1Mute,
            DeviceButton::Fader2Mute => InteractiveButtons::Fader2Mute,
            DeviceButton::Fader3Mute => InteractiveButtons::Fader3Mute,
            DeviceButton::Fader4Mute => InteractiveButtons::Fader4Mute,
            DeviceButton::Swear => InteractiveButtons::Swear,
            DeviceButton::CoughButton => InteractiveButtons::CoughButton,
            DeviceButton::EffectSelect1 => InteractiveButtons::EffectSelect1,
            DeviceButton::EffectSelect2 => InteractiveButtons::EffectSelect2,
            DeviceButton::EffectSelect3 => InteractiveButtons::EffectSelect3,
            DeviceButton::EffectSelect4 => InteractiveButtons::EffectSelect4,
            DeviceButton::EffectSelect5 => InteractiveButtons::EffectSelect5,
            DeviceButton::EffectSelect6 => InteractiveButtons::EffectSelect6,
            DeviceButton::EffectFx => InteractiveButtons::EffectFx,
            DeviceButton::EffectMegaphone => InteractiveButtons::EffectMegaphone,
            DeviceButton::EffectRobot => InteractiveButtons::EffectRobot,
            DeviceButton::EffectHardTune => InteractiveButtons::EffectHardTune,
            DeviceButton::SamplerSelectA => InteractiveButtons::SamplerSelectA,
            DeviceButton::SamplerSelectB => InteractiveButtons::SamplerSelectB,
            DeviceButton::SamplerSelectC => InteractiveButtons::SamplerSelectC,
            DeviceButton::SamplerTopLeft => InteractiveButtons::SamplerTopLeft,
            DeviceButton::SamplerTopRight => InteractiveButtons::SamplerTopRight,
            DeviceButton::SamplerBottomLeft => InteractiveButtons::SamplerBottomLeft,
            DeviceButton::SamplerBottomRight => InteractiveButtons::SamplerBottomRight,
            DeviceButton::SamplerClear => InteractiveButtons::SamplerClear,
        }
    }
}

impl From<Buttons> for DeviceButton {
    fn from(value: Buttons) -> Self {
        match value {
            Buttons::FaderA => DeviceButton::Fader1Mute,
            Buttons::FaderB => DeviceButton::Fader2Mute,
            Buttons::FaderC => DeviceButton::Fader3Mute,
            Buttons::FaderD => DeviceButton::Fader4Mute,
            Buttons::Swear => DeviceButton::Swear,
            Buttons::CoughButton => DeviceButton::CoughButton,
            Buttons::EffectSelect1 => DeviceButton::EffectSelect1,
            Buttons::EffectSelect2 => DeviceButton::EffectSelect2,
            Buttons::EffectSelect3 => DeviceButton::EffectSelect3,
            Buttons::EffectSelect4 => DeviceButton::EffectSelect4,
            Buttons::EffectSelect5 => DeviceButton::EffectSelect5,
            Buttons::EffectSelect6 => DeviceButton::EffectSelect6,
            Buttons::EffectFx => DeviceButton::EffectFx,
            Buttons::EffectMegaphone => DeviceButton::EffectMegaphone,
            Buttons::EffectRobot => DeviceButton::EffectRobot,
            Buttons::EffectHardTune => DeviceButton::EffectHardTune,
            Buttons::SamplerSelectA => DeviceButton::SamplerSelectA,
            Buttons::SamplerSelectB => DeviceButton::SamplerSelectB,
            Buttons::SamplerSelectC => DeviceButton::SamplerSelectC,
            Buttons::SamplerTopLeft => DeviceButton::SamplerTopLeft,
            Buttons::SamplerTopRight => DeviceButton::SamplerTopRight,
            Buttons::SamplerBottomLeft => DeviceButton::SamplerBottomLeft,
            Buttons::SamplerBottomRight => DeviceButton::SamplerBottomRight,
            Buttons::SamplerClear => DeviceButton::SamplerClear,
        }
    }
}
