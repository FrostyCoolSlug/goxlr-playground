use crate::device::goxlr::components::mic::mic_type::MicTypeCrate;
use crate::device::goxlr::device::GoXLR;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait LoadMicProfile {
    async fn load_mic_profile(&mut self) -> Result<()>;
}

#[async_trait]
impl LoadMicProfile for GoXLR {
    async fn load_mic_profile(&mut self) -> Result<()> {
        //self.apply_mic_gain().await?;

        Ok(())
    }
}
