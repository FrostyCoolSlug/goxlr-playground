use crate::client::Client;

use crate::commands::{
    DaemonRequest, DaemonResponse, DaemonStatus, DeviceCommand, GoXLRCommand, GoXLRCommandResponse,
};
use anyhow::bail;
use async_trait::async_trait;

#[derive(Debug)]
pub struct WebClient {
    url: String,
    status: DaemonStatus,
}

impl WebClient {
    pub fn new(url: String) -> Self {
        Self {
            url,
            status: DaemonStatus::default(),
        }
    }
}

#[async_trait]
impl Client for WebClient {
    async fn send(&mut self, request: DaemonRequest) -> anyhow::Result<()> {
        let resp = reqwest::Client::new()
            .post(&self.url)
            .json(&request)
            .send()
            .await?
            .json::<DaemonResponse>()
            .await?;

        // Should probably abstract this part, it's common between clients..
        match resp {
            DaemonResponse::Status(status) => {
                self.status = status.clone();
                Ok(())
            }
            DaemonResponse::Ok => Ok(()),
            DaemonResponse::Error(error) => bail!("{}", error),
            DaemonResponse::Command(response) => match response {
                GoXLRCommandResponse::Ok => Ok(()),
                GoXLRCommandResponse::Error(error) => bail!("{}", error),
            },
        }
    }

    async fn poll_status(&mut self) -> anyhow::Result<()> {
        self.send(DaemonRequest::GetStatus).await
    }

    async fn command(&mut self, serial: &str, command: GoXLRCommand) -> anyhow::Result<()> {
        let command = DaemonRequest::DeviceCommand(DeviceCommand {
            serial: serial.to_string(),
            command,
        });
        self.send(command).await
    }

    fn status(&self) -> &DaemonStatus {
        &self.status
    }
}
