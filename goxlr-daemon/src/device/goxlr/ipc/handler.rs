use anyhow::Result;

use goxlr_ipc::commands::{GoXLRCommand, GoXLRCommandResponse};

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::channels::IPCChannelHandler;
use crate::device::goxlr::ipc::configuration::IPCConfigurationHandler;
use crate::device::goxlr::ipc::microphone::IPCMicrophoneHandler;
use crate::device::goxlr::ipc::pages::IPCPageHandler;

pub type Response = Result<GoXLRCommandResponse>;

pub trait IPCCommandHandler {
    async fn handle_ipc_command(&mut self, command: GoXLRCommand) -> Response;
}

impl IPCCommandHandler for GoXLR {
    async fn handle_ipc_command(&mut self, command: GoXLRCommand) -> Response {
        match command {
            GoXLRCommand::Configuration(command) => self.ipc_configuration(command).await,
            GoXLRCommand::Channels(command) => self.ipc_channel(command).await,
            GoXLRCommand::Pages(command) => self.ipc_page(command).await,
            GoXLRCommand::Microphone(command) => self.ipc_microphone(command).await,
        }
    }
}
