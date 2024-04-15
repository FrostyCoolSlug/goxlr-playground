use goxlr_ipc::commands::GoXLRCommandResponse;

use crate::device::goxlr::components::pages::FaderPages;
use goxlr_ipc::commands::pages::PageCommand;

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::handler::Response;

type Command = PageCommand;

pub trait IPCPageHandler {
    async fn ipc_page(&mut self, command: Command) -> Response;
}

impl IPCPageHandler for GoXLR {
    async fn ipc_page(&mut self, command: Command) -> Response {
        match command {
            Command::LoadPage(page_number) => self.set_page(page_number as usize).await?,
            Command::AddPage => self.add_page().await?,
            Command::RemovePage(page_number) => self.remove_page(page_number as usize).await?,
            Command::SetFader(set_fader) => {
                self.set_page_fader_source(
                    set_fader.page_number as usize,
                    set_fader.fader,
                    set_fader.channel,
                )
                .await?
            }
        }

        Ok(GoXLRCommandResponse::Ok)
    }
}
