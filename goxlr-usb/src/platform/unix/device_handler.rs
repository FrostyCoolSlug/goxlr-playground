use crate::platform::unix::device::GoXLRUSB;
use crate::pnp_base::DeviceEvents;
use crate::state_tracker::GoXLRStateTracker;
use crate::{ChangeEvent, GoXLRDevice};
use anyhow::{bail, Result};
use log::debug;
use std::fmt::Error;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::time;

pub async fn spawn_device_handler(
    goxlr: GoXLRDevice,
    ready: oneshot::Sender<Result<()>>,
    event_sender: Sender<ChangeEvent>,
    device_sender: Sender<DeviceEvents>,
) {
    let device = GoXLRUSB::from_device(goxlr.clone()).await;
    let mut state = GoXLRStateTracker::new(event_sender);

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
    let mut ticker = time::interval(Duration::from_millis(20));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let states = device.get_button_states().await;
                match states {
                    Ok(states) => state.update_states(states).await,
                    Err(error) => {
                        debug!("Error Updating States: {:?}", error);
                        let _ = device_sender.send(DeviceEvents::Error(goxlr.clone())).await;

                        // Break out the loop, something bad has happened.
                        break;
                    },
                }
            }
        }
    }
}
