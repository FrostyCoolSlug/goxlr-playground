use crate::device::goxlr::device::GoXLR;
use anyhow::{bail, Result};
use goxlr_ipc::commands::{GoXLRCommandResponse, MicrophoneCommand};
use goxlr_usb::events::commands::CommandSender;
use tokio::sync::oneshot;

type Command = MicrophoneCommand;
pub type Response = Result<GoXLRCommandResponse>;
pub trait IPCMicrophoneHandler {
    async fn ipc_microphone(&mut self, command: Command) -> Response;
}

trait IPCMicrophoneHandlerLocal {
    async fn get_mic_level(&self) -> Result<f64>;
}

impl IPCMicrophoneHandler for GoXLR {
    async fn ipc_microphone(&mut self, command: Command) -> Response {
        match command {
            Command::GetMicLevel => Ok(GoXLRCommandResponse::MicLevel(self.get_mic_level().await?)),
        }
    }
}

impl IPCMicrophoneHandlerLocal for GoXLR {
    async fn get_mic_level(&self) -> Result<f64> {
        let (msg_send, msg_receive) = oneshot::channel();

        if let Some(sender) = self.command_sender.clone() {
            let command = CommandSender::GetMicLevel(msg_send);
            let _ = sender.send(command).await;

            return msg_receive.await?;
        }
        bail!("Sender Failure");
    }
}
