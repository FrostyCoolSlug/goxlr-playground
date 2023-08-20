/**
 * This is primarily a replacement for colour handling, that removes the difficulty of manually
 * building the colour array. Instead this struct can be built, stored, and altered and will
 * produce the correct output.
 */
use strum::EnumIter;

use crate::buttons::Buttons;
use crate::encoders::Encoders;
use crate::faders::Fader;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// These should not be changed, immediately consult a physician (or Frosty) if your
// GoXLR has suddenly sprouted extra colour targets!
const FADER_COUNT: usize = 4;
const MOOD_COUNT: usize = 2;
const PRESET_COUNT: usize = 6;
const ENCODER_COUNT: usize = 4;
const SAMPLE_BANK_COUNT: usize = 3;
const SAMPLE_BUTTON_COUNT: usize = 4;
const FX_BUTTON_COUNT: usize = 4;
const MIC_BUTTON_COUNT: usize = 2;

#[derive(Default, Debug, Copy, Clone)]
pub struct ColourScheme {
    pub is_legacy: bool,

    // All the Structs for all the colours..
    pub scribbles: [TwoColour; FADER_COUNT],
    pub mood: [TwoColour; MOOD_COUNT],
    pub mutes: [TwoColour; FADER_COUNT],
    pub faders: [FaderColour; FADER_COUNT],
    pub dummy1: [OneColour; 1],
    pub presets: [TwoColour; PRESET_COUNT],
    pub encoders: [ThreeColour; ENCODER_COUNT],
    pub dummy2: [OneColour; 1],
    pub sample_banks: [TwoColour; SAMPLE_BANK_COUNT],
    pub sample_buttons: [TwoColour; SAMPLE_BUTTON_COUNT],
    pub fx_buttons: [TwoColour; FX_BUTTON_COUNT],
    pub mic_buttons: [TwoColour; MIC_BUTTON_COUNT],
    pub dummy3: [TwoColour; 1], // What this is, and what this does, is anyone's guess.
}

impl ColourScheme {
    pub fn new(is_legacy: bool) -> Self {
        Self {
            is_legacy,
            ..Default::default()
        }
    }

    pub fn get_two_colour_target(&mut self, target: TwoColourTargets) -> &mut TwoColour {
        match target {
            TwoColourTargets::Scribble1 => &mut self.scribbles[0],
            TwoColourTargets::Scribble2 => &mut self.scribbles[1],
            TwoColourTargets::Scribble3 => &mut self.scribbles[2],
            TwoColourTargets::Scribble4 => &mut self.scribbles[3],
            TwoColourTargets::InternalLight => &mut self.mood[0],
            TwoColourTargets::LogoX => &mut self.mood[1],
            TwoColourTargets::Fader1Mute => &mut self.mutes[0],
            TwoColourTargets::Fader2Mute => &mut self.mutes[1],
            TwoColourTargets::Fader3Mute => &mut self.mutes[2],
            TwoColourTargets::Fader4Mute => &mut self.mutes[3],
            TwoColourTargets::EffectSelect1 => &mut self.presets[0],
            TwoColourTargets::EffectSelect2 => &mut self.presets[1],
            TwoColourTargets::EffectSelect3 => &mut self.presets[2],
            TwoColourTargets::EffectSelect4 => &mut self.presets[3],
            TwoColourTargets::EffectSelect5 => &mut self.presets[4],
            TwoColourTargets::EffectSelect6 => &mut self.presets[5],
            TwoColourTargets::SamplerSelectA => &mut self.sample_banks[0],
            TwoColourTargets::SamplerSelectB => &mut self.sample_banks[1],
            TwoColourTargets::SamplerSelectC => &mut self.sample_banks[2],
            TwoColourTargets::SamplerClear => &mut self.sample_buttons[0],
            TwoColourTargets::SamplerTopLeft => &mut self.sample_buttons[1],
            TwoColourTargets::SamplerTopRight => &mut self.sample_buttons[2],
            TwoColourTargets::SamplerBottomLeft => &mut self.sample_buttons[3],
            TwoColourTargets::SamplerBottomRight => &mut self.sample_buttons[4],
            TwoColourTargets::EffectMegaphone => &mut self.fx_buttons[0],
            TwoColourTargets::EffectRobot => &mut self.fx_buttons[1],
            TwoColourTargets::EffectHardTune => &mut self.fx_buttons[2],
            TwoColourTargets::EffectFx => &mut self.fx_buttons[3],
            TwoColourTargets::Swear => &mut self.mic_buttons[0],
            TwoColourTargets::CoughButton => &mut self.mic_buttons[1],
        }
    }

    pub fn get_fader_target(&mut self, target: Fader) -> &mut FaderColour {
        &mut self.faders[target as usize]
    }

    pub fn get_encoder_target(&mut self, target: Encoders) -> &mut ThreeColour {
        &mut self.encoders[target as usize]
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Colour {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Copy, Clone)]
pub struct OneColour {
    pub colour1: Colour,
}

#[derive(Default, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TwoColour {
    pub colour1: Colour,
    pub colour2: Colour,
}

#[derive(Default, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThreeColour {
    pub left: Colour,
    pub right: Colour,
    pub knob: Colour,
}

/// FaderColour lives separately, as it has different behaviours depending on the firmware
/// version. While we won't see them here, they'll be handled in the USB crate.
#[derive(Default, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FaderColour {
    pub colour1: Colour,
    pub colour2: Colour,
}

#[derive(Debug, Copy, Clone, EnumIter)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TwoColourTargets {
    // Scribble Bar first..
    Scribble1,
    Scribble2,
    Scribble3,
    Scribble4,

    // Mood Lighting..
    InternalLight,
    LogoX,

    // Fader Mute Buttons
    Fader1Mute,
    Fader2Mute,
    Fader3Mute,
    Fader4Mute,

    // Effect Presets Selectors
    EffectSelect1,
    EffectSelect2,
    EffectSelect3,
    EffectSelect4,
    EffectSelect5,
    EffectSelect6,

    // Sample Bank Selectors
    SamplerSelectA,
    SamplerSelectB,
    SamplerSelectC,

    // Sample Buttons
    SamplerClear,
    SamplerTopLeft,
    SamplerTopRight,
    SamplerBottomLeft,
    SamplerBottomRight,

    // FX Buttons
    EffectMegaphone,
    EffectRobot,
    EffectHardTune,
    EffectFx,

    // Finally, the Mic Buttons
    Swear,
    CoughButton,
}

impl Into<TwoColourTargets> for Buttons {
    fn into(self) -> TwoColourTargets {
        match self {
            Buttons::Fader1Mute => TwoColourTargets::Fader1Mute,
            Buttons::Fader2Mute => TwoColourTargets::Fader2Mute,
            Buttons::Fader3Mute => TwoColourTargets::Fader3Mute,
            Buttons::Fader4Mute => TwoColourTargets::Fader4Mute,
            Buttons::Swear => TwoColourTargets::Swear,
            Buttons::CoughButton => TwoColourTargets::CoughButton,
            Buttons::EffectSelect1 => TwoColourTargets::EffectSelect1,
            Buttons::EffectSelect2 => TwoColourTargets::EffectSelect2,
            Buttons::EffectSelect3 => TwoColourTargets::EffectSelect3,
            Buttons::EffectSelect4 => TwoColourTargets::EffectSelect4,
            Buttons::EffectSelect5 => TwoColourTargets::EffectSelect5,
            Buttons::EffectSelect6 => TwoColourTargets::EffectSelect6,
            Buttons::EffectFx => TwoColourTargets::EffectFx,
            Buttons::EffectMegaphone => TwoColourTargets::EffectMegaphone,
            Buttons::EffectRobot => TwoColourTargets::EffectRobot,
            Buttons::EffectHardTune => TwoColourTargets::EffectHardTune,
            Buttons::SamplerSelectA => TwoColourTargets::SamplerSelectA,
            Buttons::SamplerSelectB => TwoColourTargets::SamplerSelectB,
            Buttons::SamplerSelectC => TwoColourTargets::SamplerSelectC,
            Buttons::SamplerTopLeft => TwoColourTargets::SamplerTopLeft,
            Buttons::SamplerTopRight => TwoColourTargets::SamplerTopRight,
            Buttons::SamplerBottomLeft => TwoColourTargets::SamplerBottomLeft,
            Buttons::SamplerBottomRight => TwoColourTargets::SamplerBottomRight,
            Buttons::SamplerClear => TwoColourTargets::SamplerClear,
        }
    }
}
