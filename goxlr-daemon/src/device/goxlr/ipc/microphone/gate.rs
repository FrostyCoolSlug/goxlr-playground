use crate::device::goxlr::components::mic::gate::Gate;
use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::handler::Response;
use goxlr_ipc::commands::mic::gate::GateCommand;
use goxlr_ipc::commands::GoXLRCommandResponse;

type Command = GateCommand;
pub trait IPCMicGateHandler {
    async fn ipc_mic_gate(&mut self, command: Command) -> Response;
}

impl IPCMicGateHandler for GoXLR {
    async fn ipc_mic_gate(&mut self, command: Command) -> Response {
        match command {
            Command::SetEnabled(enabled) => self.set_gate_enabled(enabled).await?,
            Command::SetThreshold(threshold) => self.set_gate_threshold(threshold).await?,
            Command::SetAttack(attack) => self.set_gate_attack(attack).await?,
            Command::SetRelease(release) => self.set_gate_release(release).await?,
            Command::SetAttenuation(attenuation) => self.set_gate_attenuation(attenuation).await?,
        }
        Ok(GoXLRCommandResponse::Ok)
    }
}
