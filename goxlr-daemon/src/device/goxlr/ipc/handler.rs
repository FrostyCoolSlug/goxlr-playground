use anyhow::Result;
use async_trait::async_trait;

use goxlr_ipc::commands::{Channels, GoXLRCommand, GoXLRCommandResponse};

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::channels::IPCChannelHandler;
use crate::device::goxlr::ipc::pages::IPCPageHandler;

pub type Response = Result<GoXLRCommandResponse>;

#[async_trait]
pub trait IPCCommandHandler {
    async fn handle_ipc_command(&mut self, command: GoXLRCommand) -> Response;
}

#[async_trait]
impl IPCCommandHandler for GoXLR {
    async fn handle_ipc_command(&mut self, command: GoXLRCommand) -> Response {
        match command {
            GoXLRCommand::Channels(command) => {
                let Channels { channel, command } = command;
                self.ipc_channel(channel, command).await
            }
            GoXLRCommand::Pages(command) => self.ipc_page(command).await,
        }
    }
}
