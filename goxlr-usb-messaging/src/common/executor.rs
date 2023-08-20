use crate::goxlr::commands::Command;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait ExecutableGoXLR {
    async fn request_data(&mut self, command: Command, body: &[u8]) -> Result<Vec<u8>> {
        self.perform_request(command, body, false).await
    }

    async fn perform_request(
        &mut self,
        command: Command,
        body: &[u8],
        is_retry_attempt: bool,
    ) -> Result<Vec<u8>>;
}

#[async_trait]
pub(crate) trait InitialisableGoXLR {
    async fn initialise(&mut self) -> Result<()>;
}
