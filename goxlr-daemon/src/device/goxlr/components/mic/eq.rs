use anyhow::{bail, Result};
use ritelinked::LinkedHashMap;
use strum::IntoEnumIterator;

use crate::device::goxlr::device::GoXLR;
use goxlr_shared::eq_frequencies::{Frequencies, MiniFrequencies};
use goxlr_shared::microphone::{MicEffectKeys, MicParamKeys};
use goxlr_usb::events::commands::BasicResultCommand;

pub trait MicEq {
    async fn set_full_mic_eq_freq(&mut self, freq: Frequencies, value: f32) -> Result<()>;
    async fn set_full_mic_eq_gain(&mut self, freq: Frequencies, gain: i8) -> Result<()>;

    async fn set_mini_mic_eq_freq(&mut self, freq: MiniFrequencies, value: f32) -> Result<()>;
    async fn set_mini_mic_eq_gain(&mut self, freq: MiniFrequencies, gain: i8) -> Result<()>;
}

impl MicEq for GoXLR {
    async fn set_full_mic_eq_freq(&mut self, freq: Frequencies, value: f32) -> Result<()> {
        let min = self.get_frequency_min(freq);
        let max = self.get_frequency_max(freq);
        if !(min..=max).contains(&value) {
            bail!("Invalid Value {}, expected: {} - {}", value, min, max);
        }
        self.mic_profile.equalizer[freq].frequency = value;

        let key = MicEffectKeys::from_eq_freq(freq);
        let map = LinkedHashMap::from_iter([(key, Self::freq_as_i32(value))]);
        let command = BasicResultCommand::SetMicEffects(map);
        self.send_no_result(command).await
    }

    async fn set_full_mic_eq_gain(&mut self, freq: Frequencies, gain: i8) -> Result<()> {
        if !(-9..=9).contains(&gain) {
            bail!("EQ Gain should be between -9 and 9");
        }
        self.mic_profile.equalizer[freq].gain = gain;

        let map = LinkedHashMap::from_iter([(MicEffectKeys::from_eq_gain(freq), gain as i32)]);
        let command = BasicResultCommand::SetMicEffects(map);
        self.send_no_result(command).await
    }

    async fn set_mini_mic_eq_freq(&mut self, freq: MiniFrequencies, value: f32) -> Result<()> {
        let min = self.get_mini_frequency_min(freq);
        let max = self.get_mini_frequency_max(freq);
        if !(min..=max).contains(&value) {
            bail!("Invalid Value {}, expected: {} - {}", value, min, max);
        }
        self.mic_profile.equalizer_mini[freq].frequency = value;

        let map = LinkedHashMap::from_iter([(MicParamKeys::from_eq_freq(freq), value)]);
        let command = BasicResultCommand::SetMicParams(map);
        self.send_no_result(command).await
    }

    async fn set_mini_mic_eq_gain(&mut self, freq: MiniFrequencies, gain: i8) -> Result<()> {
        if !(-9..=9).contains(&gain) {
            bail!("EQ Gain should be between -9 and 9");
        }
        self.mic_profile.equalizer_mini[freq].gain = gain;

        let map = LinkedHashMap::from_iter([(MicParamKeys::from_eq_gain(freq), gain as f32)]);
        let command = BasicResultCommand::SetMicParams(map);
        self.send_no_result(command).await
    }
}

pub(crate) trait MicEqCrate {
    fn get_eq_keys(&self) -> LinkedHashMap<MicEffectKeys, i32>;
    fn get_mini_eq_keys(&self) -> LinkedHashMap<MicParamKeys, f32>;
}

impl MicEqCrate for GoXLR {
    fn get_eq_keys(&self) -> LinkedHashMap<MicEffectKeys, i32> {
        let mut map = LinkedHashMap::new();
        for freq in Frequencies::iter() {
            map.insert(
                MicEffectKeys::from_eq_freq(freq),
                Self::freq_as_i32(self.mic_profile.equalizer[freq].frequency),
            );
            map.insert(
                MicEffectKeys::from_eq_gain(freq),
                self.mic_profile.equalizer[freq].gain as i32,
            );
        }
        map
    }

    fn get_mini_eq_keys(&self) -> LinkedHashMap<MicParamKeys, f32> {
        let mut map = LinkedHashMap::new();
        for freq in MiniFrequencies::iter() {
            map.insert(
                MicParamKeys::from_eq_freq(freq),
                self.mic_profile.equalizer_mini[freq].frequency,
            );
            map.insert(
                MicParamKeys::from_eq_gain(freq),
                self.mic_profile.equalizer_mini[freq].gain as f32,
            );
        }
        map
    }
}

trait MicEqLocal {
    fn get_frequency_min(&self, freq: Frequencies) -> f32;
    fn get_frequency_max(&self, freq: Frequencies) -> f32;

    fn get_mini_frequency_min(&self, freq: MiniFrequencies) -> f32;
    fn get_mini_frequency_max(&self, freq: MiniFrequencies) -> f32;

    fn freq_as_i32(freq: f32) -> i32;
}

impl MicEqLocal for GoXLR {
    /// Find out the minimal acceptable value for a frequency selection is, ensuring that
    /// we do not overlap with the value below.
    fn get_frequency_min(&self, freq: Frequencies) -> f32 {
        match freq {
            Frequencies::Eq31h => 30.,
            Frequencies::Eq63h => f32::max(
                30.,
                self.mic_profile.equalizer[Frequencies::Eq31h].frequency,
            ),
            Frequencies::Eq125h => f32::max(
                30.,
                self.mic_profile.equalizer[Frequencies::Eq63h].frequency,
            ),
            Frequencies::Eq250h => f32::max(
                30.,
                self.mic_profile.equalizer[Frequencies::Eq125h].frequency,
            ),
            Frequencies::Eq500h => f32::max(
                300.,
                self.mic_profile.equalizer[Frequencies::Eq250h].frequency,
            ),
            Frequencies::Eq1kh => f32::max(
                300.,
                self.mic_profile.equalizer[Frequencies::Eq500h].frequency,
            ),
            Frequencies::Eq2kh => f32::max(
                300.,
                self.mic_profile.equalizer[Frequencies::Eq1kh].frequency,
            ),
            Frequencies::Eq4kh => f32::max(
                2000.,
                self.mic_profile.equalizer[Frequencies::Eq2kh].frequency,
            ),
            Frequencies::Eq8kh => f32::max(
                2000.,
                self.mic_profile.equalizer[Frequencies::Eq4kh].frequency,
            ),
            Frequencies::Eq16kh => f32::max(
                2000.,
                self.mic_profile.equalizer[Frequencies::Eq8kh].frequency,
            ),
        }
    }

    /// Find out the Maximum Acceptable Frequency for a frequency selection, ensuring we don't
    /// overlap the item above.
    fn get_frequency_max(&self, freq: Frequencies) -> f32 {
        match freq {
            Frequencies::Eq31h => f32::min(
                300.0,
                self.mic_profile.equalizer[Frequencies::Eq63h].frequency,
            ),
            Frequencies::Eq63h => f32::min(
                300.0,
                self.mic_profile.equalizer[Frequencies::Eq125h].frequency,
            ),
            Frequencies::Eq125h => f32::min(
                300.0,
                self.mic_profile.equalizer[Frequencies::Eq250h].frequency,
            ),
            Frequencies::Eq250h => f32::min(
                300.0,
                self.mic_profile.equalizer[Frequencies::Eq500h].frequency,
            ),
            Frequencies::Eq500h => f32::min(
                2000.0,
                self.mic_profile.equalizer[Frequencies::Eq1kh].frequency,
            ),
            Frequencies::Eq1kh => f32::min(
                2000.0,
                self.mic_profile.equalizer[Frequencies::Eq2kh].frequency,
            ),
            Frequencies::Eq2kh => f32::min(
                2000.0,
                self.mic_profile.equalizer[Frequencies::Eq4kh].frequency,
            ),
            Frequencies::Eq4kh => f32::min(
                18000.0,
                self.mic_profile.equalizer[Frequencies::Eq8kh].frequency,
            ),
            Frequencies::Eq8kh => f32::min(
                18000.0,
                self.mic_profile.equalizer[Frequencies::Eq16kh].frequency,
            ),
            Frequencies::Eq16kh => 18000.0,
        }
    }

    fn get_mini_frequency_min(&self, freq: MiniFrequencies) -> f32 {
        match freq {
            MiniFrequencies::Eq90h => 30.0,
            MiniFrequencies::Eq250h => 100.0,
            MiniFrequencies::Eq500h => 310.0,
            MiniFrequencies::Eq1kh => 800.0,
            MiniFrequencies::Eq3kh => 2600.0,
            MiniFrequencies::Eq8kh => 5100.0,
        }
    }

    fn get_mini_frequency_max(&self, freq: MiniFrequencies) -> f32 {
        match freq {
            MiniFrequencies::Eq90h => 90.0,
            MiniFrequencies::Eq250h => 300.0,
            MiniFrequencies::Eq500h => 800.0,
            MiniFrequencies::Eq1kh => 2500.0,
            MiniFrequencies::Eq3kh => 5000.0,
            MiniFrequencies::Eq8kh => 18000.0,
        }
    }

    fn freq_as_i32(freq: f32) -> i32 {
        (24.0 * (freq / 20.0).log2()).round() as i32
    }
}
