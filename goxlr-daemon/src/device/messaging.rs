use tokio::sync::oneshot;

use goxlr_ipc::commands::{
    DaemonCommand, DaemonResponse, DaemonStatus, GoXLRCommand, GoXLRCommandResponse,
};

pub enum DeviceMessage {
    GetStatus(oneshot::Sender<DaemonStatus>),
    RunDaemon(DaemonCommand, oneshot::Sender<DaemonResponse>),
    RunDevice(String, GoXLRCommand, oneshot::Sender<GoXLRCommandResponse>),
}
