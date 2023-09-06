use anyhow::{bail, Context, Result};
use log::{debug, error, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};
use tokio::sync::mpsc;
use tokio::{join, task};

use crate::device::device_manager::start_device_manager;
use crate::servers::ipc_server::{bind_socket, spawn_ipc_server};
use crate::stop::Stop;

mod device;
mod servers;
mod settings;
mod stop;

#[tokio::main]
async fn main() -> Result<()> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        ConfigBuilder::new().build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .context("Could not configure the logger")?;

    // Spawn the Shutdown Handler..
    let shutdown = Stop::new();

    // Create the Global Manager Channels..
    let (manager_send, manager_recv) = mpsc::channel(32);

    // Prepare the IPC Socket..
    let ipc_socket = bind_socket().await;
    if ipc_socket.is_err() {
        error!("Error Starting Daemon: ");
        bail!("{}", ipc_socket.err().unwrap());
    }

    let task = task::spawn(start_device_manager(manager_recv, shutdown.clone()));

    let ipc_socket = ipc_socket.unwrap();
    let communications_handle = tokio::spawn(spawn_ipc_server(
        ipc_socket,
        manager_send.clone(),
        shutdown.clone(),
    ));

    // We're going to go to sleep, then trigger the shutdown..
    // sleep(Duration::from_secs(5)).await;
    // shutdown.trigger();

    let _ = join!(task, communications_handle);

    debug!("Should be done!");
    Ok(())
}
