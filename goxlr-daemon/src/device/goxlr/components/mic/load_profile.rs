use crate::device::goxlr::components::mic::mic_eq::MicEqCrate;
use crate::device::goxlr::components::mic::mic_type::MicTypeCrate;
use anyhow::Result;
use goxlr_shared::device::DeviceType;
use goxlr_usb::events::commands::BasicResultCommand;
use ritelinked::LinkedHashMap;

use crate::device::goxlr::device::GoXLR;

pub trait LoadMicProfile {
    async fn load_mic_profile(&mut self) -> Result<()>;
}

impl LoadMicProfile for GoXLR {
    async fn load_mic_profile(&mut self) -> Result<()> {
        let device_type = if let Some(device) = &self.device {
            device.device_type
        } else {
            DeviceType::Mini
        };
        self.apply_mic_gain().await?;

        let mut mic_params = LinkedHashMap::new();
        let mut mic_effects = LinkedHashMap::new();

        if device_type == DeviceType::Mini {
            self.get_mini_eq_keys().iter().for_each(|(key, value)| {
                mic_params.insert(*key, *value);
            });
        } else {
            self.get_eq_keys().iter().for_each(|(key, value)| {
                mic_effects.insert(*key, *value);
            })
        }

        if !mic_params.is_empty() {
            let command = BasicResultCommand::SetMicParams(mic_params);
            self.send_no_result(command).await?;
        }

        if !mic_effects.is_empty() {
            let command = BasicResultCommand::SetMicEffects(mic_effects);
            self.send_no_result(command).await?;
        }

        //self.apply_mic_gain().await?;

        Ok(())
    }
}
