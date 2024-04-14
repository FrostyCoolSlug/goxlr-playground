use crate::device::goxlr::components::mic::eq::MicEqCrate;
use crate::device::goxlr::components::mic::gate::GateCrate;
use crate::device::goxlr::components::mic::r#type::MicTypeCrate;
use anyhow::Result;
use goxlr_usb::events::commands::BasicResultCommand;
use ritelinked::LinkedHashMap;

use crate::device::goxlr::device::GoXLR;

pub trait LoadMicProfile {
    async fn load_mic_profile(&mut self) -> Result<()>;
}

impl LoadMicProfile for GoXLR {
    async fn load_mic_profile(&mut self) -> Result<()> {
        self.apply_mic_gain().await?;

        let mut mic_params = LinkedHashMap::new();
        let mut mic_effects = LinkedHashMap::new();

        // Load the Equaliser...
        mic_params.extend(self.get_eq_mini_values());
        mic_effects.extend(self.get_eq_values());

        // Load the Configured Gate..
        mic_params.extend(self.get_gate_mini_values());
        mic_effects.extend(self.get_gate_values());

        let command = BasicResultCommand::SetMicParams(mic_params);
        self.send_no_result(command).await?;

        let command = BasicResultCommand::SetMicEffects(mic_effects);
        self.send_no_result(command).await?;

        Ok(())
    }
}
