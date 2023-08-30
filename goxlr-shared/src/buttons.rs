use crate::colours::TwoColourTargets;
use crate::faders::Fader;
use crate::interaction::InteractiveButtons;
use enum_map::Enum;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum, EnumIter)]
pub enum Buttons {
    // Fader Mute Buttons
    FaderA,
    FaderB,
    FaderC,
    FaderD,

    // Cough / Bleep Buttons
    Swear,
    CoughButton,

    // FX Buttons
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

impl Buttons {
    pub fn from_fader(fader: Fader) -> Buttons {
        match fader {
            Fader::A => Buttons::FaderA,
            Fader::B => Buttons::FaderB,
            Fader::C => Buttons::FaderC,
            Fader::D => Buttons::FaderD,
        }
    }
}

impl From<InteractiveButtons> for Buttons {
    fn from(value: InteractiveButtons) -> Self {
        match value {
            InteractiveButtons::Fader1Mute => Buttons::FaderA,
            InteractiveButtons::Fader2Mute => Buttons::FaderB,
            InteractiveButtons::Fader3Mute => Buttons::FaderC,
            InteractiveButtons::Fader4Mute => Buttons::FaderD,
            InteractiveButtons::Swear => Buttons::Swear,
            InteractiveButtons::CoughButton => Buttons::CoughButton,
            InteractiveButtons::EffectSelect1 => Buttons::EffectSelect1,
            InteractiveButtons::EffectSelect2 => Buttons::EffectSelect2,
            InteractiveButtons::EffectSelect3 => Buttons::EffectSelect3,
            InteractiveButtons::EffectSelect4 => Buttons::EffectSelect4,
            InteractiveButtons::EffectSelect5 => Buttons::EffectSelect5,
            InteractiveButtons::EffectSelect6 => Buttons::EffectSelect6,
            InteractiveButtons::EffectFx => Buttons::EffectFx,
            InteractiveButtons::EffectMegaphone => Buttons::EffectMegaphone,
            InteractiveButtons::EffectRobot => Buttons::EffectRobot,
            InteractiveButtons::EffectHardTune => Buttons::EffectHardTune,
            InteractiveButtons::SamplerSelectA => Buttons::SamplerSelectA,
            InteractiveButtons::SamplerSelectB => Buttons::SamplerSelectB,
            InteractiveButtons::SamplerSelectC => Buttons::SamplerSelectC,
            InteractiveButtons::SamplerTopLeft => Buttons::SamplerTopLeft,
            InteractiveButtons::SamplerTopRight => Buttons::SamplerTopRight,
            InteractiveButtons::SamplerBottomLeft => Buttons::SamplerBottomLeft,
            InteractiveButtons::SamplerBottomRight => Buttons::SamplerBottomRight,
            InteractiveButtons::SamplerClear => Buttons::SamplerClear,
        }
    }
}

impl From<TwoColourTargets> for Buttons {
    fn from(value: TwoColourTargets) -> Self {
        match value {
            TwoColourTargets::Fader1Mute => Buttons::FaderA,
            TwoColourTargets::Fader2Mute => Buttons::FaderB,
            TwoColourTargets::Fader3Mute => Buttons::FaderC,
            TwoColourTargets::Fader4Mute => Buttons::FaderD,
            TwoColourTargets::EffectSelect1 => Buttons::EffectSelect1,
            TwoColourTargets::EffectSelect2 => Buttons::EffectSelect2,
            TwoColourTargets::EffectSelect3 => Buttons::EffectSelect3,
            TwoColourTargets::EffectSelect4 => Buttons::EffectSelect4,
            TwoColourTargets::EffectSelect5 => Buttons::EffectSelect5,
            TwoColourTargets::EffectSelect6 => Buttons::EffectSelect6,
            TwoColourTargets::SamplerSelectA => Buttons::SamplerSelectA,
            TwoColourTargets::SamplerSelectB => Buttons::SamplerSelectB,
            TwoColourTargets::SamplerSelectC => Buttons::SamplerSelectC,
            TwoColourTargets::SamplerClear => Buttons::SamplerClear,
            TwoColourTargets::SamplerTopLeft => Buttons::SamplerTopLeft,
            TwoColourTargets::SamplerTopRight => Buttons::SamplerTopRight,
            TwoColourTargets::SamplerBottomLeft => Buttons::SamplerBottomLeft,
            TwoColourTargets::SamplerBottomRight => Buttons::SamplerBottomRight,
            TwoColourTargets::EffectMegaphone => Buttons::EffectMegaphone,
            TwoColourTargets::EffectRobot => Buttons::EffectRobot,
            TwoColourTargets::EffectHardTune => Buttons::EffectHardTune,
            TwoColourTargets::EffectFx => Buttons::EffectFx,
            TwoColourTargets::Swear => Buttons::Swear,
            TwoColourTargets::CoughButton => Buttons::CoughButton,
            _ => panic!("Attempted to Lookup Two Colour on a non-button!"),
        }
    }
}

/// Defines potential inactive button behaviours
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum InactiveButtonBehaviour {
    /// This Dimms the Active Colour.
    DimActive,

    /// This Dimms the inactive Colour.
    DimInactive,

    /// This brightly displays the inactive colour.
    InactiveColour,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ButtonActiveState {
    Active,
    Inactive,
}
