use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait LoadProfile {
    async fn load_mic_profile(&mut self) -> Result<()>;
}
