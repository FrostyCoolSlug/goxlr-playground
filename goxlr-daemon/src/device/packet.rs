use anyhow::{anyhow, Context, Result};
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use goxlr_ipc::commands::{DaemonRequest, DaemonResponse};

use crate::device::messaging::DeviceCommand;

pub type Messanger = Sender<DeviceCommand>;
type Response = Result<DaemonResponse>;

/// This is pretty similar to the GoXLR Utility, as very little really needs to change here.
pub async fn handle_packet(request: DaemonRequest, sender: Messanger) -> Response {
    // Ok, we just match the request, and send it off where it needs to go..
    match request {
        DaemonRequest::Ping => Ok(DaemonResponse::Ok),
        DaemonRequest::GetStatus => {
            let (tx, rx) = oneshot::channel();

            sender
                .send(DeviceCommand::GetStatus(tx))
                .await
                .map_err(|e| anyhow!(e.to_string()))
                .context("Failed to send message to device manager")?;

            let result = rx.await.context("Error from device manager")?;
            Ok(DaemonResponse::Status(result))
        }
        DaemonRequest::Daemon(daemon_command) => {
            let (tx, rx) = oneshot::channel();
            sender
                .send(DeviceCommand::RunDaemon(daemon_command, tx))
                .await
                .map_err(|e| anyhow!(e.to_string()))
                .context("Failed to send message to device manager")?;

            rx.await.context("Error from device manager")?;
            Ok(DaemonResponse::Ok)
        }
        DaemonRequest::DeviceCommand(serial, command) => {
            let (tx, rx) = oneshot::channel();
            sender
                .send(DeviceCommand::RunDevice(serial, command, tx))
                .await
                .map_err(|e| anyhow!(e.to_string()))
                .context("Failed to send message to device manager")?;

            let result = rx.await.context("Error from Device Manager")?;
            Ok(DaemonResponse::Command(result))
        }
    }
}
