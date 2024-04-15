use crate::device::goxlr::components::mic::mic_type::MicType;
use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::handler::Response;
use goxlr_ipc::commands::mic::setup::SetupCommand;
use goxlr_ipc::commands::GoXLRCommandResponse;

type Command = SetupCommand;
pub trait IPCMicSetupHandler {
    async fn ipc_mic_setup(&mut self, command: Command) -> Response;
}

impl IPCMicSetupHandler for GoXLR {
    async fn ipc_mic_setup(&mut self, command: Command) -> Response {
        match command {
            Command::SetMicType(mic_type) => self.set_mic_type(mic_type).await?,
            Command::SetMicGain(gain) => self.set_mic_gain(gain).await?,
        }
        Ok(GoXLRCommandResponse::Ok)
    }
}
