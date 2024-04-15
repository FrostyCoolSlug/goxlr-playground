use crate::cli::PageCommands;
use anyhow::Result;
use goxlr_ipc::client::Client;
use goxlr_ipc::commands::pages::{PageCommand, SetFader};
use goxlr_ipc::commands::{DaemonRequest, DeviceCommand, GoXLRCommand};

pub async fn handle_pages(
    serial: String,
    mut client: Box<dyn Client>,
    command: PageCommands,
) -> Result<()> {
    match command {
        PageCommands::SetPage { page_number } => {
            let command = PageCommand::LoadPage(page_number);
            let command = GoXLRCommand::Pages(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });
            client.send(command).await?;
        }
        PageCommands::AddPage => {
            let command = PageCommand::AddPage;
            let command = GoXLRCommand::Pages(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });
            client.send(command).await?;
        }
        PageCommands::RemovePage { page_number } => {
            let command = PageCommand::RemovePage(page_number);
            let command = GoXLRCommand::Pages(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });
            client.send(command).await?;
        }
        PageCommands::SetFader {
            page_number,
            fader,
            channel,
        } => {
            let command = PageCommand::SetFader(SetFader {
                page_number,
                fader,
                channel,
            });
            let command = GoXLRCommand::Pages(command);
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });
            client.send(command).await?;
        }
    }

    Ok(())
}
