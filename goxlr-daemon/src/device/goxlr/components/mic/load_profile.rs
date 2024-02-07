use crate::device::goxlr::components::mic::mic_type::MicTypeCrate;
use crate::device::goxlr::device::GoXLR;
use anyhow::Result;

pub trait LoadMicProfile {
    async fn load_mic_profile(&mut self) -> Result<()>;
}

impl LoadMicProfile for GoXLR {
    async fn load_mic_profile(&mut self) -> Result<()> {
        //self.apply_mic_gain().await?;

        Ok(())
    }
}
