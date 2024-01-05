/*
   The primary device manager, this is responsible for most of the general workings of the daemon
*/

use std::collections::HashMap;
use std::time::Duration;

use log::{debug, error, info, warn};
use tokio::sync::oneshot::error::RecvError;
use tokio::sync::{mpsc, oneshot};
use tokio::{join, select, task, time};

use goxlr_ipc::commands::{
    DaemonResponse, DaemonStatus, DeviceStatus, GoXLRCommand, GoXLRCommandResponse, Profiles,
};
use goxlr_profile::Profile;
use goxlr_usb::runners::pnp::PnPDeviceMessage;
use goxlr_usb::runners::pnp::{start_pnp_runner, PnPConfiguration};
use goxlr_usb::USBLocation;

use crate::device::device_manager::ManagerMessage::{Execute, GetConfig};
use crate::device::goxlr::device::start_goxlr;
use crate::device::goxlr::device_config::GoXLRDeviceConfiguration;
use crate::device::messaging::DeviceMessage;
use crate::stop::Stop;

struct DeviceManager {
    /// Used for Devices sending messages back to the Manager
    device_receiver: mpsc::Receiver<RunnerMessage>,
    device_sender: mpsc::Sender<RunnerMessage>,

    /// Current State of all attached devices
    states: HashMap<USBLocation, DeviceState>,

    /// Currently registered device serials
    serials: HashMap<String, USBLocation>,

    /// Shutdown Signaller
    shutdown: Stop,

    /// Simple bool to help track shutdown
    stopping: bool,
}

impl DeviceManager {
    pub fn new(shutdown: Stop) -> Self {
        let (device_sender, device_receiver) = mpsc::channel(128);

        Self {
            device_receiver,
            device_sender,

            states: HashMap::default(),
            serials: HashMap::default(),
            shutdown,
            stopping: false,
        }
    }

    pub async fn run(&mut self, mut message_receiver: mpsc::Receiver<DeviceMessage>) {
        info!("[DeviceManager] Starting Device Manager..");
        let (pnp_send, pnp_recv) = oneshot::channel();
        let (device_send, mut device_recv) = mpsc::channel(32);

        let pnp_configuration = PnPConfiguration {
            stop_signal: pnp_recv,
            device_sender: device_send,
        };

        let pnp = task::spawn(start_pnp_runner(pnp_configuration));

        // Ticker for handling error states..
        let mut ticker = time::interval(Duration::from_millis(500));

        loop {
            select! {
                Some(message) = message_receiver.recv() => {
                    self.handle_command(message).await;
                }
                Some(device) = device_recv.recv() => {
                    match device {
                        PnPDeviceMessage::Attached(device) => {
                            info!("[DeviceManager] Received Device: {:?}", device);
                            self.add_device(device).await;
                        }
                        PnPDeviceMessage::Removed(device) => {
                            info!("[DeviceManager] Device Removed: {:?}", device);
                            self.remove_device(device).await;
                        },
                    }
                },
                Some(message) = self.device_receiver.recv() => {
                    debug!("[DeviceManager] Received State Change from GoXLR: {:?}", message);
                    match message {
                        RunnerMessage::UpdateState(device, state) => {
                            self.update_state(device, state);
                        }
                        RunnerMessage::Error(device) => {
                            self.handle_error(device);
                        },
                    }
                },
                _ = self.shutdown.recv() => {
                    let _ = pnp_send.send(());
                    self.stopping = true;
                    break;
                }
                _ = ticker.tick() => {
                    self.check_devices().await;
                }
            }
        }

        // If we get here, we've been asked to stop, move into a shutdown loop that waits for
        // all the devices to finish up..
        loop {
            select! {
                Some(message) = self.device_receiver.recv() => {
                    debug!("[DeviceManager-SD] Received State Change from GoXLR..");
                    match message {
                        RunnerMessage::UpdateState(device, state) => {
                            self.update_state(device, state);
                        }
                        RunnerMessage::Error(device) => {
                            self.handle_error(device);
                        },
                    }
                    if self.devices_stopped() {
                        break;
                    }
                }
            }
        }
        info!("[DeviceManager] All Devices stopped, waiting for PnP Handler to terminate.");
        let _ = join!(pnp);
        info!("[DeviceManager] Everything shut down, terminating");
    }

    async fn add_device(&mut self, device: USBLocation) {
        let stop = Stop::new();
        let (manager_send, manager_recv) = mpsc::channel(64);

        // Ok, we have a new device, we need to add it and set it up..
        let config = GoXLRDeviceConfiguration {
            stop: stop.clone(),
            device,
            manager_sender: self.device_sender.clone(),
            manager_recv,
        };

        let state = DeviceState {
            stop,
            state: RunnerState::Starting,
            messanger: manager_send,
        };

        self.states.insert(device, state);
        task::spawn(start_goxlr(config, self.shutdown.clone()));
    }

    async fn remove_device(&mut self, device: USBLocation) {
        if let Some(status) = &mut self.states.get_mut(&device) {
            if let RunnerState::Running(_) = &status.state {
                // We're running, trigger a stop and set us to stopping..
                status.stop.trigger();
                status.state = RunnerState::Stopping;

                // Remove our Serial Tracking for this device..
                self.serials.retain(|_, dev| *dev != device);

                // Return here, and wait for the Stopper to handle the stop.
                return;
            }
        }

        // If we're not already running, we should just nuke knowledge of the device..
        self.serials.retain(|_, dev| *dev != device);
        self.states.retain(|dev, _| *dev != device);
    }

    async fn check_devices(&mut self) {
        let mut refresh = vec![];
        // We need to see if any of our devices are in an error state, if so, reset them..
        for (location, state) in &mut self.states {
            if state.state == RunnerState::Error {
                refresh.push(*location);
            }
        }

        // Refresh any devices that are in an error state..
        for device in refresh {
            self.add_device(device).await;
        }
    }

    fn update_state(&mut self, device: USBLocation, state: RunnerState) {
        if let RunnerState::Running(serial) = &state {
            info!(
                "[DeviceManager]{} Serial {} entered Running State",
                device, serial
            );
            self.serials.insert(serial.to_owned(), device);
        }

        if let Some(current) = self.states.get_mut(&device) {
            if state == RunnerState::Stopped {
                debug!("[DeviceManager]{} Device Terminated", device);
                // If we get here, the device has stopped, we should clear it..
                self.serials.retain(|_, dev| *dev != device);

                // If we're in a 'Stopping' state, we're prepping for removal..
                if current.state == RunnerState::Stopping && self.stopping {
                    self.states.remove(&device);
                } else {
                    // We've stopped, but we're not supposed to, that's an error.
                    current.state = RunnerState::Error;
                }
                return;
            }

            current.state = state;
            return;
        }

        error!(
            "[DeviceManager]{} Attempted to Update State on non-existing device!",
            device
        );
    }

    fn handle_error(&mut self, device: USBLocation) {
        // Called if something has gone wrong with the device and it may need a reset..
        // flag it as ERROR, and let the error handling catch it later.
        if let Some(current) = self.states.get_mut(&device) {
            // Errors should internally break loops, so we don't need to call stop..
            current.state = RunnerState::Error;
        }

        // If we're tracking a serial for this device, we need to remove it from the list..
        self.serials.retain(|_, dev| *dev != device);
    }

    fn devices_stopped(&self) -> bool {
        for state in self.states.values() {
            let current_state = &state.state;

            if *current_state != RunnerState::Error && *current_state != RunnerState::Stopped {
                return false;
            }
        }
        true
    }

    async fn handle_command(&self, command: DeviceMessage) {
        match command {
            DeviceMessage::GetStatus(tx) => {
                debug!("Getting Status..");
                let mut status = DaemonStatus { devices: vec![] };

                for (serial, usb) in &self.serials {
                    if let Some(device) = self.states.get(usb) {
                        let (cmd_tx, cmd_rx) = oneshot::channel();

                        let result = device.messanger.send(GetConfig(cmd_tx)).await;
                        if let Err(e) = result {
                            warn!("Unable to Fetch Device Config: {}", e);
                        }

                        let response = cmd_rx.await;
                        match response {
                            Ok(profile) => {
                                status.devices.push(DeviceStatus {
                                    serial: serial.clone(),
                                    config: profile,
                                });
                            }
                            Err(error) => {
                                debug!(
                                    "Error Fetching Profile Information for {}: {}",
                                    serial, error
                                );
                                continue;
                            }
                        }
                    }
                }

                debug!("Status: {:#?}", status);
                let _ = tx.send(status);
            }
            DeviceMessage::RunDaemon(command, tx) => {
                let _ = tx.send(DaemonResponse::Ok);
            }
            DeviceMessage::RunDevice(serial, command, tx) => {
                if let Some(usb) = self.serials.get(&*serial) {
                    if let Some(device) = self.states.get(usb) {
                        let (cmd_tx, cmd_rx) = oneshot::channel();

                        let result = device.messanger.send(Execute(command, cmd_tx)).await;
                        if let Err(e) = result {
                            let _ = tx.send(GoXLRCommandResponse::Error(e.to_string()));
                            return;
                        }

                        let response = cmd_rx.await;
                        match response {
                            Ok(result) => {
                                let _ = tx.send(result);
                            }
                            Err(error) => {
                                let _ = tx.send(GoXLRCommandResponse::Error(error.to_string()));
                            }
                        }
                    }
                } else {
                    let error = format!("Device {} not found", serial);
                    let _ = tx.send(GoXLRCommandResponse::Error(error));
                }
            }
        }
    }
}

pub async fn start_device_manager(message_receiver: mpsc::Receiver<DeviceMessage>, shutdown: Stop) {
    let mut manager = DeviceManager::new(shutdown);
    manager.run(message_receiver).await;
}

#[derive(Debug)]
pub enum ManagerMessage {
    GetConfig(oneshot::Sender<Profiles>),
    Execute(GoXLRCommand, oneshot::Sender<GoXLRCommandResponse>),
}

struct DeviceState {
    stop: Stop,
    state: RunnerState,
    messanger: mpsc::Sender<ManagerMessage>,
}

#[derive(Debug)]
pub enum RunnerMessage {
    UpdateState(USBLocation, RunnerState),
    Error(USBLocation),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RunnerState {
    Starting,
    Running(String),
    Stopping,
    Stopped,
    Error,
}
