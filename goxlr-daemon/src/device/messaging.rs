use tokio::sync::oneshot;

use goxlr_ipc::commands::{
    DaemonCommand, DaemonResponse, DaemonStatus, GoXLRCommand, GoXLRCommandResponse,
};

pub enum DeviceCommand {
    GetDaemonStatus(oneshot::Sender<DaemonStatus>),
    RunDaemonCommand(DaemonCommand, oneshot::Sender<DaemonResponse>),
    RunDeviceCommand(String, GoXLRCommand, oneshot::Sender<GoXLRCommandResponse>),
}
