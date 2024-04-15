use crate::device::goxlr::components::mic::compressor::Compressor;
use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::handler::Response;
use goxlr_ipc::commands::mic::compressor::CompressorCommand;
use goxlr_ipc::commands::GoXLRCommandResponse;

type Command = CompressorCommand;
pub trait IPCMicCompressorHandler {
    async fn ipc_mic_compressor(&mut self, command: Command) -> Response;
}

impl IPCMicCompressorHandler for GoXLR {
    async fn ipc_mic_compressor(&mut self, command: Command) -> Response {
        match command {
            Command::SetThreshold(threshold) => self.set_compressor_threshold(threshold).await?,
            Command::SetRatio(ratio) => self.set_compressor_ratio(ratio).await?,
            Command::SetAttack(attack) => self.set_compressor_attack(attack).await?,
            Command::SetRelease(release) => self.set_compressor_release(release).await?,
            Command::SetMakeupGain(gain) => self.set_compressor_makeup_gain(gain).await?,
        }

        Ok(GoXLRCommandResponse::Ok)
    }
}
