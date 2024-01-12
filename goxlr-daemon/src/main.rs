use actix_web::dev::ServerHandle;
use anyhow::{bail, Context, Result};
use goxlr_ipc::commands::HttpSettings;
use log::{debug, error, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};
use tokio::sync::{broadcast, mpsc};
use tokio::{join, task};

use crate::device::device_manager::start_device_manager;
use crate::servers::http_server::spawn_http_server;
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
    let ipc_socket = ipc_socket.unwrap();
    let communications_handle = tokio::spawn(spawn_ipc_server(
        ipc_socket,
        manager_send.clone(),
        shutdown.clone(),
    ));

    // Prepare the HTTP Server..
    let http_settings = HttpSettings {
        enabled: true,
        bind_address: "localhost".to_string(),
        cors_enabled: false,
        port: 14564,
    };

    let (httpd_tx, httpd_rx) = tokio::sync::oneshot::channel();
    let (broadcast_tx, broadcast_rx) = broadcast::channel(16);
    drop(broadcast_rx);

    tokio::spawn(spawn_http_server(
        manager_send.clone(),
        httpd_tx,
        broadcast_tx.clone(),
        http_settings,
    ));
    let http_server = httpd_rx.await?;

    // We're going to go to sleep, then trigger the shutdown..
    // sleep(Duration::from_secs(5)).await;
    // shutdown.trigger();

    let task = task::spawn(start_device_manager(
        manager_recv,
        shutdown.clone(),
        broadcast_tx.clone(),
    ));
    let _ = join!(task, communications_handle);

    join!(http_server.stop(true));

    debug!("Should be done!");
    Ok(())
}
