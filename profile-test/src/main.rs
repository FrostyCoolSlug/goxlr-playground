mod device;

use anyhow::Result;

use crate::device::Device;
use goxlr_profile::Profile;
use goxlr_usb::device::base::FullGoXLRDevice;
use goxlr_usb::device::{find_devices, from_device};
use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() -> Result<()> {
    let profile = Profile::default();

    // Grab the Devices..
    let devices = find_devices();

    // Create the Message Queues...
    let (disconnect_sender, _disconnect_receiver) = mpsc::channel(32);
    let (event_sender, event_receiver) = mpsc::channel(32);

    let device = devices[0].clone();
    let mut handled_device = from_device(device, disconnect_sender, event_sender)?;

    let (serial_number, _) = handled_device.get_serial_number()?;
    handled_device.set_unique_identifier(serial_number);

    // Ok, Send our profile to the device, see what happens :D
    let mut device = Device::new(handled_device, event_receiver, profile).await?;
    device.load_profile().await;

    // Run this up, and sit on the thread.
    let runtime = task::spawn(run_handler(device));
    let _ = tokio::join!(runtime);

    Ok(())
}

async fn run_handler(mut device: Device) {
    device.run_handler().await;
}
