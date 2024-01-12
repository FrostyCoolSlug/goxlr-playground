use crate::client::Client;
use crate::clients::ipc::ipc_socket::Socket;
use crate::commands::{
    DaemonRequest, DaemonResponse, DaemonStatus, DeviceCommand, GoXLRCommand, GoXLRCommandResponse,
};
use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use interprocess::local_socket::tokio::LocalSocketStream;
use interprocess::local_socket::NameTypeSupport;

static SOCKET_PATH: &str = "/tmp/goxlr.socket";
static NAMED_PIPE: &str = "@goxlr.socket";

#[derive(Debug)]
pub struct IPCClient {
    socket: Socket<DaemonResponse, DaemonRequest>,
    status: DaemonStatus,
}

impl IPCClient {
    pub async fn connect() -> Result<Self> {
        let connection = LocalSocketStream::connect(match NameTypeSupport::query() {
            NameTypeSupport::OnlyPaths | NameTypeSupport::Both => SOCKET_PATH,
            NameTypeSupport::OnlyNamespaced => NAMED_PIPE,
        })
        .await?;
        let socket: Socket<DaemonResponse, DaemonRequest> = Socket::new(connection);

        Ok(IPCClient::start(socket))
    }

    fn start(socket: Socket<DaemonResponse, DaemonRequest>) -> Self {
        Self {
            socket,
            status: DaemonStatus::default(),
        }
    }
}

#[async_trait]
impl Client for IPCClient {
    async fn send(&mut self, request: DaemonRequest) -> Result<()> {
        self.socket
            .send(request)
            .await
            .context("Failed to send a command to the GoXLR daemon process")?;
        let result = self
            .socket
            .read()
            .await
            .context("Failed to retrieve the command result from the GoXLR daemon process")?
            .context("Failed to parse the command result from the GoXLR daemon process")?;

        match result {
            DaemonResponse::Status(status) => {
                self.status = status.clone();
                Ok(())
            }
            DaemonResponse::Ok => Ok(()),
            DaemonResponse::Err(error) => bail!("{}", error),
            DaemonResponse::Patch(_) => bail!("Unexpected PATCH"),
            DaemonResponse::Command(response) => match response {
                GoXLRCommandResponse::Ok => Ok(()),
                GoXLRCommandResponse::Error(error) => Err(anyhow!("{}", error)),
            },
        }
    }

    async fn poll_status(&mut self) -> Result<()> {
        self.send(DaemonRequest::GetStatus).await
    }

    async fn command(&mut self, serial: &str, command: GoXLRCommand) -> Result<()> {
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
