use goxlr_ipc::commands::configuration::ConfigurationCommand;
use goxlr_ipc::commands::GoXLRCommandResponse;
use crate::device::goxlr::components::buttons::ButtonHandlers;
use crate::device::goxlr::components::pages::FaderPages;
use crate::device::goxlr::components::submix::SubMix;

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::handler::Response;

type Command = ConfigurationCommand;

pub trait IPCConfigurationHandler {
    async fn ipc_configuration(&mut self, command: Command) -> Response;
}

impl IPCConfigurationHandler for GoXLR {
    async fn ipc_configuration(&mut self, command: Command) -> Response {
        match command {
            Command::SubMixEnabled(enabled) => {
                self.set_sub_mix_enabled(enabled).await?;
                
                Ok(GoXLRCommandResponse::Ok)
            }
            Command::ButtonHoldTime(time) => {
                self.set_button_hold_time(time).await?;
                Ok(GoXLRCommandResponse::Ok)
            }
            Command::ChangePageWithButtons(enabled) => {
                self.set_change_page_with_buttons(enabled).await?;
                Ok(GoXLRCommandResponse::Ok)
            }
        }
    }
}
