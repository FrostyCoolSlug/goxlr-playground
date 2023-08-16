use anyhow::{bail, Result};
use log::{debug, error, warn};
use tokio::sync::{mpsc, oneshot};
use tokio::{join, select, task};

use goxlr_usb_messaging::runners::device::DeviceMessage;
use goxlr_usb_messaging::runners::device::{start_usb_device_runner, GoXLRUSBConfiguration};

use crate::device::device_manager::{RunnerMessage, RunnerState};
use crate::device::goxlr::device_config::GoXLRDeviceConfiguration;
use crate::stop::Stop;

struct GoXLR {
    config: GoXLRDeviceConfiguration,
    shutdown: Stop,
}

impl GoXLR {
    pub fn new(config: GoXLRDeviceConfiguration, shutdown: Stop) -> Self {
        Self { config, shutdown }
    }

    pub async fn initialise(&mut self) -> Result<()> {
        debug!("[GoXLR]{} Initialising Device..", self.config.device);

        let error = false;
        if error {
            bail!("Error Occurred!");
        }

        debug!("[GoXLR]{} Initialisation Complete..", self.config.device);
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        debug!("[GoXLR]{} Starting Event Loop", self.config.device);

        // Prepare all the messaging queues that are needed..
        let (event_send, mut event_recv) = mpsc::channel(128);
        let (stop_send, stop_recv) = oneshot::channel();
        let (ready_send, ready_recv) = oneshot::channel();

        let configuration = GoXLRUSBConfiguration {
            device: self.config.device,
            events: event_send,
            stop: stop_recv,
        };
        let runner = task::spawn(start_usb_device_runner(configuration, ready_send));

        let messenger = match ready_recv.await {
            Ok(recv) => recv,
            Err(e) => {
                bail!("Error on Starting Receiver, aborting: {}", e);
            }
        };

        // Let the device runner know we're up and running (TODO: REQUEST SERIAL!)
        let serial = String::from("000000000000");
        let run_msg = RunnerMessage::UpdateState(self.config.device, RunnerState::Running(serial));
        let _ = self.config.manager_sender.send(run_msg).await;

        loop {
            select! {
                Some(event) = event_recv.recv() => {
                    debug!("[GoXLR]{} Event: {:?}", self.config.device, event);
                    match event {
                        DeviceMessage::Error => {
                            warn!("[GoXLR]{} Error Sent back from Handler, bail!", self.config.device);
                            break;
                        }
                        DeviceMessage::Event(event) => {
                            debug!("[GoXLR]{} Event: {:?}", self.config.device, event);
                        }
                    }
                }
                _ = self.shutdown.recv() => {
                    debug!("[GoXLR]{} Shutdown Triggered!", self.config.device);
                    break;
                }
                _ = self.config.stop.recv() => {
                    debug!("[GoXLR]{} Device Disconnected!", self.config.device);
                    break;
                }
            }
        }

        // Our loop has been broken, let the device know we're done..
        let _ = stop_send.send(());
        debug!("[GoXLR]{} Event Loop Ended", self.config.device);

        debug!(
            "[GoXLR]{} Waiting for Device Runner to stop..",
            self.config.device
        );
        let _ = join!(runner);

        debug!("[GoXLR]{} Runner Stopped", self.config.device);
        let run_msg = RunnerMessage::UpdateState(self.config.device, RunnerState::Stopped);
        let _ = self.config.manager_sender.send(run_msg).await;
        debug!("[GoXLR]{} Device Runtime Ended..", self.config.device);

        Ok(())
    }
}

pub async fn start_goxlr(config: GoXLRDeviceConfiguration, shutdown: Stop) {
    // Prepare an error handler, in case something goes wrong during init / runtime..
    let sender = config.manager_sender.clone();
    let error_msg = RunnerMessage::Error(config.device);

    let mut device = GoXLR::new(config, shutdown);
    if let Err(error) = device.initialise().await {
        error!("Failed to initialise, not spawning runtime: {}", error);
        let _ = sender.send(error_msg).await;
        return;
    }

    if let Err(error) = device.run().await {
        error!("Error during device runtime: {}", error);
        let _ = sender.send(error_msg).await;
    }
}
