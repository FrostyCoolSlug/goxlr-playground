use anyhow::{bail, Result};
use ritelinked::LinkedHashMap;

use goxlr_shared::gate::GateTimes;
use goxlr_shared::microphone::{MicEffectKeys, MicParamKeys};
use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::device::GoXLR;

static GATE_ATTENUATION: [i8; 26] = [
    -6, -7, -8, -9, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -20, -21, -22, -23, -24, -25,
    -26, -27, -28, -30, -32, -61,
];

pub trait Gate {
    async fn set_gate_enabled(&mut self, enabled: bool) -> Result<()>;
    async fn set_gate_threshold(&mut self, threshold: i8) -> Result<()>;
    async fn set_gate_attack(&mut self, attack: GateTimes) -> Result<()>;
    async fn set_gate_release(&mut self, release: GateTimes) -> Result<()>;
    async fn set_gate_attenuation(&mut self, attenuation: u8) -> Result<()>;
}

impl Gate for GoXLR {
    async fn set_gate_enabled(&mut self, enabled: bool) -> Result<()> {
        self.mic_profile.gate.enabled = enabled;

        let effect = LinkedHashMap::from_iter([(MicEffectKeys::GateEnabled, enabled as i32)]);
        let command = BasicResultCommand::SetMicEffects(effect);

        self.send_no_result(command).await
    }

    async fn set_gate_threshold(&mut self, threshold: i8) -> Result<()> {
        if !(-59..=0).contains(&threshold) {
            bail!("Gate Threshold must be between -59 and 0");
        }

        self.mic_profile.gate.threshold = threshold;

        let effect = LinkedHashMap::from_iter([(MicEffectKeys::GateThreshold, threshold as i32)]);
        let command = BasicResultCommand::SetMicEffects(effect);
        self.send_no_result(command).await?;

        let param = LinkedHashMap::from_iter([(MicParamKeys::GateThreshold, threshold as f32)]);
        let command = BasicResultCommand::SetMicParams(param);
        self.send_no_result(command).await
    }

    async fn set_gate_attack(&mut self, attack: GateTimes) -> Result<()> {
        self.mic_profile.gate.attack = attack;

        let gate_attack = attack as u8;

        let effect = LinkedHashMap::from_iter([(MicEffectKeys::GateAttack, gate_attack as i32)]);
        let command = BasicResultCommand::SetMicEffects(effect);
        self.send_no_result(command).await?;

        let param = LinkedHashMap::from_iter([(MicParamKeys::GateAttack, gate_attack as f32)]);
        let command = BasicResultCommand::SetMicParams(param);
        self.send_no_result(command).await
    }

    async fn set_gate_release(&mut self, release: GateTimes) -> Result<()> {
        self.mic_profile.gate.release = release;

        let gate_release = release as u8;

        let effect = LinkedHashMap::from_iter([(MicEffectKeys::GateRelease, gate_release as i32)]);
        let command = BasicResultCommand::SetMicEffects(effect);
        self.send_no_result(command).await?;

        let param = LinkedHashMap::from_iter([(MicParamKeys::GateRelease, gate_release as f32)]);
        let command = BasicResultCommand::SetMicParams(param);
        self.send_no_result(command).await
    }

    async fn set_gate_attenuation(&mut self, attenuation: u8) -> Result<()> {
        if attenuation > 100 {
            bail!("Gate Attenuation must be a percentage");
        }

        self.mic_profile.gate.attenuation = attenuation;

        let key = MicEffectKeys::GateAttenuation;
        let effect = LinkedHashMap::from_iter([(key, self.get_gate_attenuation())]);
        let command = BasicResultCommand::SetMicEffects(effect);
        self.send_no_result(command).await?;

        let key = MicParamKeys::GateAttenuation;
        let param = LinkedHashMap::from_iter([(key, attenuation as f32)]);
        let command = BasicResultCommand::SetMicParams(param);
        self.send_no_result(command).await
    }
}

pub(crate) trait GateCrate {
    fn get_gate_values(&self) -> LinkedHashMap<MicEffectKeys, i32>;
    fn get_gate_mini_values(&self) -> LinkedHashMap<MicParamKeys, f32>;
}

impl GateCrate for GoXLR {
    fn get_gate_values(&self) -> LinkedHashMap<MicEffectKeys, i32> {
        let mut map = LinkedHashMap::new();

        // Grab some variables..
        let enabled = self.mic_profile.gate.enabled as i32;
        let threshold = self.mic_profile.gate.threshold as i32;
        let attack = self.mic_profile.gate.attack as i32;
        let release = self.mic_profile.gate.release as i32;

        // Fill out all the Gate Values...
        map.insert(MicEffectKeys::GateMode, 2_i32);
        map.insert(MicEffectKeys::GateEnabled, enabled);
        map.insert(MicEffectKeys::GateThreshold, threshold);
        map.insert(MicEffectKeys::GateAttack, attack);
        map.insert(MicEffectKeys::GateRelease, release);
        map.insert(MicEffectKeys::GateAttenuation, self.get_gate_attenuation());

        map
    }

    fn get_gate_mini_values(&self) -> LinkedHashMap<MicParamKeys, f32> {
        let mut map = LinkedHashMap::new();

        // Grab some variables...
        let threshold = self.mic_profile.gate.threshold as f32;
        let attack = self.mic_profile.gate.attack as u8 as f32;
        let release = self.mic_profile.gate.release as u8 as f32;
        let attenuation = self.mic_profile.gate.attenuation as f32;

        // Fill out all the Gate Values..
        map.insert(MicParamKeys::GateThreshold, threshold);
        map.insert(MicParamKeys::GateAttack, attack);
        map.insert(MicParamKeys::GateRelease, release);
        map.insert(MicParamKeys::GateAttenuation, attenuation);

        map
    }
}

trait GateLocal {
    fn get_gate_attenuation(&self) -> i32;
}

impl GateLocal for GoXLR {
    fn get_gate_attenuation(&self) -> i32 {
        let percent = self.mic_profile.gate.attenuation;
        let index = percent as f32 * 0.24;

        if percent > 99 {
            return GATE_ATTENUATION[25] as i32;
        }

        GATE_ATTENUATION[index as usize] as i32
    }
}
