use anyhow::Result;

use goxlr_ipc::client::Client;
use goxlr_ipc::commands::channels::ChannelCommand;
use goxlr_ipc::commands::{Channels, DaemonRequest, DeviceCommand, GoXLRCommand};
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
            let command = ChannelCommand::Volume(volume);
            let command = GoXLRCommand::Channels(Channels { channel, command });
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
        ChannelCommands::Mute { mute_state } => {
            let command = ChannelCommand::Mute(mute_state);
            let command = GoXLRCommand::Channels(Channels { channel, command });
            let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

            client.send(command).await?;
        }
    }

    Ok(())
}
