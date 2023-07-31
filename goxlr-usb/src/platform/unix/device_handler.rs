use crate::platform::unix::device::GoXLRUSB;
use crate::state_tracker::GoXLRStateTracker;
use crate::{ChangeEvent, GoXLRDevice};
use anyhow::Result;
use log::debug;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::time;

pub async fn spawn_device_handler(
    goxlr: GoXLRDevice,
    ready: oneshot::Sender<Result<()>>,
    event_sender: Sender<ChangeEvent>,
) {
    let device = GoXLRUSB::from_device(goxlr).await;
    let state = GoXLRStateTracker::new(event_sender);

    let mut device = match device {
        Ok(device) => device,
        Err(error) => {
            let _ = ready.send(Err(error));
            return;
        }
    };

    if let Err(error) = device.initialise().await {
        let _ = ready.send(Err(error));
        return;
    }

    // Ok, loading and initialising the device is good, send the message back and begin polling.
    let _ = ready.send(Ok(()));

    // Create an interval for polling the device status..
    let mut ticker = time::interval(Duration::from_millis(500));
    loop {
        tokio::select! {
            _ = ticker.tick() => {
                debug!("Tick..");
                // Poll Status

            }
        }
    }
}
