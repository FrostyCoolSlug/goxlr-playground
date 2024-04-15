use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::handler::Response;
use crate::device::goxlr::ipc::microphone::compressor::IPCMicCompressorHandler;
use crate::device::goxlr::ipc::microphone::equaliser::IPCMicEqualiserHandler;
use crate::device::goxlr::ipc::microphone::gate::IPCMicGateHandler;
use crate::device::goxlr::ipc::microphone::setup::IPCMicSetupHandler;
use anyhow::bail;
use goxlr_ipc::commands::mic::MicrophoneCommand;
use goxlr_ipc::commands::GoXLRCommandResponse;
use goxlr_usb::events::commands::CommandSender;
use tokio::sync::oneshot;

mod compressor;
mod equaliser;
mod gate;
mod setup;

type Command = MicrophoneCommand;
pub trait IPCMicrophoneHandler {
    async fn ipc_microphone(&mut self, command: Command) -> Response;
}

impl IPCMicrophoneHandler for GoXLR {
    async fn ipc_microphone(&mut self, command: Command) -> Response {
        match command {
            Command::Setup(command) => self.ipc_mic_setup(command).await,
            Command::Equaliser(command) => self.ipc_mic_equaliser(command).await,
            Command::Compressor(command) => self.ipc_mic_compressor(command).await,
            Command::Gate(command) => self.ipc_mic_gate(command).await,

            Command::GetMicLevel => Ok(GoXLRCommandResponse::MicLevel(self.get_mic_level().await?)),
        }
    }
}

trait IPCMicrophoneHandlerLocal {
    async fn get_mic_level(&self) -> anyhow::Result<f64>;
}

impl IPCMicrophoneHandlerLocal for GoXLR {
    async fn get_mic_level(&self) -> anyhow::Result<f64> {
        let (msg_send, msg_receive) = oneshot::channel();

        if let Some(sender) = self.command_sender.clone() {
            let command = CommandSender::GetMicLevel(msg_send);
            let _ = sender.send(command).await;

            return msg_receive.await?;
        }
        bail!("Sender Failure");
    }
}
