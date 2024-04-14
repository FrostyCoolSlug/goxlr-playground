use crate::device::goxlr::device::GoXLR;
use anyhow::{bail, Result};
use goxlr_shared::compressor::{CompressorAttackTime, CompressorRatio, CompressorReleaseTime};
use goxlr_shared::microphone::{MicEffectKeys, MicParamKeys};
use goxlr_usb::events::commands::BasicResultCommand;
use ritelinked::LinkedHashMap;

pub trait Compressor {
    async fn set_compressor_threshold(&mut self, threshold: i8) -> Result<()>;
    async fn set_compressor_ratio(&mut self, ratio: CompressorRatio) -> Result<()>;
    async fn set_compressor_attack(&mut self, attack: CompressorAttackTime) -> Result<()>;
    async fn set_compressor_release(&mut self, release: CompressorReleaseTime) -> Result<()>;
    async fn set_compressor_makeup_gain(&mut self, makeup_gain: i8) -> Result<()>;
}

impl Compressor for GoXLR {
    async fn set_compressor_threshold(&mut self, threshold: i8) -> Result<()> {
        if !(-40..=0).contains(&threshold) {
            bail!("Compressor Threshold must be between -40 and 0 dB");
        }
        self.mic_profile.compressor.threshold = threshold;

        let key = MicEffectKeys::CompressorThreshold;
        let effect = LinkedHashMap::from_iter([(key, threshold as i32)]);
        let command = BasicResultCommand::SetMicEffects(effect);
        self.send_no_result(command).await?;

        let key = MicParamKeys::CompressorThreshold;
        let param = LinkedHashMap::from_iter([(key, threshold as f32)]);
        let command = BasicResultCommand::SetMicParams(param);
        self.send_no_result(command).await
    }

    async fn set_compressor_ratio(&mut self, ratio: CompressorRatio) -> Result<()> {
        self.mic_profile.compressor.ratio = ratio;

        let key = MicEffectKeys::CompressorRatio;
        let effect = LinkedHashMap::from_iter([(key, ratio as i32)]);
        let command = BasicResultCommand::SetMicEffects(effect);
        self.send_no_result(command).await?;

        let key = MicParamKeys::CompressorRatio;
        let param = LinkedHashMap::from_iter([(key, ratio as i8 as f32)]);
        let command = BasicResultCommand::SetMicParams(param);
        self.send_no_result(command).await
    }

    async fn set_compressor_attack(&mut self, attack: CompressorAttackTime) -> Result<()> {
        self.mic_profile.compressor.attack = attack;

        let key = MicEffectKeys::CompressorAttack;
        let effect = LinkedHashMap::from_iter([(key, attack as i32)]);
        let command = BasicResultCommand::SetMicEffects(effect);
        self.send_no_result(command).await?;

        let key = MicParamKeys::CompressorAttack;
        let param = LinkedHashMap::from_iter([(key, attack as i8 as f32)]);
        let command = BasicResultCommand::SetMicParams(param);
        self.send_no_result(command).await
    }

    async fn set_compressor_release(&mut self, release: CompressorReleaseTime) -> Result<()> {
        self.mic_profile.compressor.release = release;

        let key = MicEffectKeys::CompressorRelease;
        let effect = LinkedHashMap::from_iter([(key, release as i32)]);
        let command = BasicResultCommand::SetMicEffects(effect);
        self.send_no_result(command).await?;

        let key = MicParamKeys::CompressorRelease;
        let param = LinkedHashMap::from_iter([(key, release as i8 as f32)]);
        let command = BasicResultCommand::SetMicParams(param);
        self.send_no_result(command).await
    }

    async fn set_compressor_makeup_gain(&mut self, makeup_gain: i8) -> Result<()> {
        if !(-6..=24).contains(&makeup_gain) {
            bail!("Makeup Gain should be between -6 and 24dB");
        }
        self.mic_profile.compressor.makeup_gain = makeup_gain;

        let key = MicEffectKeys::CompressorMakeUpGain;
        let effect = LinkedHashMap::from_iter([(key, makeup_gain as i32)]);
        let command = BasicResultCommand::SetMicEffects(effect);
        self.send_no_result(command).await?;

        let key = MicParamKeys::CompressorMakeUpGain;
        let param = LinkedHashMap::from_iter([(key, makeup_gain as f32)]);
        let command = BasicResultCommand::SetMicParams(param);
        self.send_no_result(command).await
    }
}

pub(crate) trait CompressorCrate {
    fn get_compressor_values(&self) -> LinkedHashMap<MicEffectKeys, i32>;
    fn get_compressor_mini_values(&self) -> LinkedHashMap<MicParamKeys, f32>;
}

impl CompressorCrate for GoXLR {
    fn get_compressor_values(&self) -> LinkedHashMap<MicEffectKeys, i32> {
        let ratio = self.mic_profile.compressor.ratio as i32;
        let attack = self.mic_profile.compressor.attack as i32;
        let release = self.mic_profile.compressor.release as i32;
        let makeup_gain = self.mic_profile.compressor.makeup_gain as i32;

        let mut map = LinkedHashMap::new();
        map.insert(MicEffectKeys::MicCompSelect, 1_i32);
        map.insert(MicEffectKeys::CompressorRatio, ratio);
        map.insert(MicEffectKeys::CompressorAttack, attack);
        map.insert(MicEffectKeys::CompressorRelease, release);
        map.insert(MicEffectKeys::CompressorMakeUpGain, makeup_gain);

        map
    }

    fn get_compressor_mini_values(&self) -> LinkedHashMap<MicParamKeys, f32> {
        let ratio = self.mic_profile.compressor.ratio as i32 as f32;
        let attack = self.mic_profile.compressor.attack as i32 as f32;
        let release = self.mic_profile.compressor.release as i32 as f32;
        let makeup_gain = self.mic_profile.compressor.makeup_gain as f32;

        let mut map = LinkedHashMap::new();
        map.insert(MicParamKeys::CompressorRatio, ratio);
        map.insert(MicParamKeys::CompressorAttack, attack);
        map.insert(MicParamKeys::CompressorRelease, release);
        map.insert(MicParamKeys::CompressorMakeUpGain, makeup_gain);

        map
    }
}
