use crate::device::goxlr::components::mic::eq::MicEq;
use goxlr_ipc::commands::mic::equaliser::{
    EqualiserCommand, FullEqualiserCommand, MiniEqualiserCommand,
};
use goxlr_ipc::commands::GoXLRCommandResponse;

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::handler::Response;

type Command = EqualiserCommand;
type MiniCommand = MiniEqualiserCommand;
type FullCommand = FullEqualiserCommand;
pub trait IPCMicEqualiserHandler {
    async fn ipc_mic_equaliser(&mut self, command: Command) -> Response;
}

impl IPCMicEqualiserHandler for GoXLR {
    async fn ipc_mic_equaliser(&mut self, command: Command) -> Response {
        match command {
            Command::Mini(command) => self.ipc_mic_equaliser_mini(command).await,
            Command::Full(command) => self.ipc_mic_equaliser_full(command).await,
        }
    }
}

trait IPCMicEqualiserHandlerLocal {
    async fn ipc_mic_equaliser_mini(&mut self, command: MiniCommand) -> Response;
    async fn ipc_mic_equaliser_full(&mut self, command: FullCommand) -> Response;
}

impl IPCMicEqualiserHandlerLocal for GoXLR {
    async fn ipc_mic_equaliser_mini(&mut self, command: MiniCommand) -> Response {
        match command {
            MiniCommand::SetFrequency(params) => {
                self.set_mini_mic_eq_freq(params.base, params.frequency)
                    .await?
            }
            MiniCommand::SetGain(params) => {
                self.set_mini_mic_eq_gain(params.base, params.gain).await?
            }
        }
        Ok(GoXLRCommandResponse::Ok)
    }

    async fn ipc_mic_equaliser_full(&mut self, command: FullCommand) -> Response {
        match command {
            FullCommand::SetFrequency(params) => {
                self.set_full_mic_eq_freq(params.base, params.frequency)
                    .await?
            }
            FullCommand::SetGain(params) => {
                self.set_full_mic_eq_gain(params.base, params.gain).await?
            }
        }

        Ok(GoXLRCommandResponse::Ok)
    }
}
