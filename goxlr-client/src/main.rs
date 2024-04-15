use anyhow::{bail, Result};
use clap::Parser;

use goxlr_ipc::client::Client;
use goxlr_ipc::clients::ipc::ipc_client::IPCClient;

use crate::cli::{Cli, SubCommands};
use crate::processors::channel::handle_channels;
use crate::processors::microphone::handle_microphone;
use crate::processors::pages::handle_pages;

mod cli;
mod processors;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut client: Box<dyn Client>;
    client = Box::new(IPCClient::connect().await?);
    client.poll_status().await?;

    if cli.status_json {
        println!("{:#?}", client.status());
    }

    let serial;

    if client.status().devices.is_empty() {
        bail!("No GoXLR Devices Detected");
    }

    if client.status().devices.len() > 1 && cli.serial.is_none() {
        bail!("More than one device detected, specify device with --serial");
    }

    if let Some(cli_serial) = cli.serial {
        serial = cli_serial;
    } else {
        // If we get here, we already check for an empty device list, so we should be
        // fine to straight up unwrap the first key.
        serial = client.status().devices.keys().next().unwrap().clone();
    }

    if let Some(command) = cli.command {
        match command {
            SubCommands::Microphone { command } => {
                handle_microphone(serial, client, command).await?;
            }

            SubCommands::Channels { channel, command } => {
                handle_channels(serial, client, channel, command).await?;
            }
            SubCommands::Pages { command } => {
                handle_pages(serial, client, command).await?;
            }
        }
    }

    Ok(())
}
