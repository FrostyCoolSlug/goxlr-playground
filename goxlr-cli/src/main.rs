use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use goxlr_ipc::client::Client;
use interprocess::local_socket::tokio::LocalSocketStream;
use interprocess::local_socket::NameTypeSupport;

use goxlr_ipc::clients::ipc::ipc_client::IPCClient;
use goxlr_ipc::clients::ipc::ipc_socket::Socket;
use goxlr_ipc::commands::{DaemonRequest, DaemonResponse};

use crate::cli::{Cli, SubCommands};
use crate::processors::channel::handle_channels;

mod cli;
mod processors;

static SOCKET_PATH: &str = "/tmp/goxlr.socket";
static NAMED_PIPE: &str = "@goxlr.socket";

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let client: Box<dyn Client>;

    // Build the Client..
    let connection = LocalSocketStream::connect(match NameTypeSupport::query() {
        NameTypeSupport::OnlyPaths | NameTypeSupport::Both => SOCKET_PATH,
        NameTypeSupport::OnlyNamespaced => NAMED_PIPE,
    })
    .await
    .context("Unable to connect to the GoXLR daemon Process")?;

    let socket: Socket<DaemonResponse, DaemonRequest> = Socket::new(connection);
    client = Box::new(IPCClient::new(socket));

    let serial = String::from("S201200586CQK");

    if cli.status_json {
        println!("{:?}", client.status());
    }

    if let Some(command) = cli.command {
        match command {
            SubCommands::Channels { channel, command } => {
                handle_channels(serial, client, channel, command).await?;
            }
        }
    }

    Ok(())
}
