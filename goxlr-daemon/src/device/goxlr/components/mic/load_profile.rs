use crate::device::goxlr::components::mic::mic_type::MicTypeCrate;
use anyhow::Result;

use crate::device::goxlr::device::GoXLR;

pub trait LoadMicProfile {
    async fn load_mic_profile(&mut self) -> Result<()>;
}

impl LoadMicProfile for GoXLR {
    async fn load_mic_profile(&mut self) -> Result<()> {
        self.apply_mic_gain().await?;

        //self.apply_mic_gain().await?;

        Ok(())
    }
}
