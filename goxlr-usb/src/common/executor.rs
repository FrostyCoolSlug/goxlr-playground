use crate::goxlr::commands::Command;
use anyhow::{bail, Result};
use async_trait::async_trait;
use log::warn;

#[async_trait]
pub(crate) trait ExecutableGoXLR {
    async fn request_data(&mut self, command: Command, body: &[u8]) -> Result<Vec<u8>> {
        match self.perform_request(command, body).await {
            Ok(result) => return Ok(result),
            Err(error) => {
                warn!("Error Executing Command, attempting recovery: {}", error);

                // Attempt Recovery..
                if let Err(error) = self.perform_recovery().await {
                    self.perform_stop().await;
                    return Err(error);
                }

                let result = self.perform_request(command, body).await;
                match result {
                    Ok(result) => Ok(result),
                    Err(error) => {
                        self.perform_stop().await;
                        return Err(error);
                    }
                }
            }
        }
    }

    async fn perform_request(&mut self, command: Command, body: &[u8]) -> Result<Vec<u8>>;
    async fn perform_recovery(&mut self) -> Result<()>;
    async fn perform_stop(&mut self);
}

pub(crate) trait InitialisableGoXLR {
    async fn initialise(&mut self) -> Result<()>;
}
