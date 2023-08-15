use anyhow::{Context, Result};
use log::{debug, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};
use std::time::Duration;
use tokio::time::sleep;
use tokio::{join, task};

use crate::device::device_manager::start_device_manager;
use crate::stop::Stop;

mod device;
mod primary_worker;
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

    let task = task::spawn(start_device_manager(shutdown.clone()));

    // We're going to go to sleep, then trigger the shutdown..
    // sleep(Duration::from_secs(5)).await;
    // shutdown.trigger();

    let _ = join!(task);

    debug!("Should be done!");

    // let task = task::spawn(run_worker());
    // join!(task);
    //
    // let profile = Profile::default();

    // Grab the Devices..
    // let devices = find_devices();

    // Create the Message Queues...
    // let (disconnect_sender, _disconnect_receiver) = mpsc::channel(32);
    // let (event_sender, event_receiver) = mpsc::channel(32);

    // let device = devices[0].clone();
    // let mut handled_device = from_device(device, disconnect_sender, event_sender)?;

    // let (serial_number, _) = handled_device.get_serial_number()?;
    // handled_device.set_unique_identifier(serial_number);

    // Ok, Send our profile to the device, see what happens :D
    // let mut device = Device::new(handled_device, event_receiver, profile).await?;
    // device.load_profile().await;

    // Run this up, and sit on the thread.
    // let runtime = task::spawn(run_handler(device));
    // let _ = tokio::join!(runtime);

    Ok(())
}
