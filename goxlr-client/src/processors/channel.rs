use anyhow::Result;

use goxlr_ipc::client::Client;
use goxlr_ipc::commands::channels::ChannelCommands as IPCChannelCommand;
use goxlr_ipc::commands::channels::{ChannelVolume, MuteCommand};
use goxlr_ipc::commands::{DaemonRequest, DeviceCommand, GoXLRCommand};

use crate::cli::{ChannelCommands, FaderCommands, VolumeCommands};

pub async fn handle_channels(
    serial: String,
    mut client: Box<dyn Client>,
    command: ChannelCommands,
) -> Result<()> {
    match command {
        ChannelCommands::Volumes { channel, command } => match command {
            VolumeCommands::Volume { volume } => {
                let command = ChannelVolume { channel, volume };
                let command = IPCChannelCommand::Volume(command);
                let command = GoXLRCommand::Channels(command);
                let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

                client.send(command).await?;
            }
        },
        ChannelCommands::Faders { channel, command } => match command {
            FaderCommands::Mute { state } => {
                let command = MuteCommand { channel, state };
                let command = IPCChannelCommand::Mute(command);
                let command = GoXLRCommand::Channels(command);
                let command = DaemonRequest::DeviceCommand(DeviceCommand { serial, command });

                client.send(command).await?;
            }
        },
        ChannelCommands::SubMix { .. } => {}
    }

    Ok(())
}
