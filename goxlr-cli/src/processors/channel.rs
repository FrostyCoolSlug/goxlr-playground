use anyhow::Result;

use goxlr_ipc::client::Client;
use goxlr_ipc::commands::channels::ChannelCommand;
use goxlr_ipc::commands::{DaemonRequest, GoXLRCommand};
use goxlr_shared::faders::FaderSources;

use crate::cli::ChannelCommands;

pub async fn handle_channels(
    serial: String,
    mut client: Box<dyn Client>,
    channel: FaderSources,
    command: ChannelCommands,
) -> Result<()> {
    match command {
        ChannelCommands::Volume { volume } => {
            // Build the Command..
            let command = ChannelCommand::SetVolume(volume);
            let command = GoXLRCommand::Channels(channel, command);
            let command = DaemonRequest::DeviceCommand(serial, command);

            client.send(command).await?;
        }
        ChannelCommands::Mute { state } => {
            let command = ChannelCommand::SetMute(state);
            let command = GoXLRCommand::Channels(channel, command);
            let command = DaemonRequest::DeviceCommand(serial, command);

            client.send(command).await?;
        }
    }

    Ok(())
}
