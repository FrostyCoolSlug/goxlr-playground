/*
   The primary device manager, this is responsible for most of the general workings of the daemon
*/

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use json_patch::diff;
use log::{debug, error, info, warn};
use tokio::sync::broadcast::Sender;
use tokio::sync::{mpsc, oneshot};
use tokio::{join, select, task, time};

use goxlr_ipc::commands::{
    DaemonResponse, DaemonStatus, DeviceStatus, GoXLRCommand, GoXLRCommandResponse, Profiles,
};
use goxlr_usb::runners::pnp::PnPDeviceMessage;
use goxlr_usb::runners::pnp::{start_pnp_runner, PnPConfiguration};
use goxlr_usb::USBLocation;

use crate::device::device_manager::ManagerMessage::{Execute, GetConfig};
use crate::device::goxlr::device::start_goxlr;
use crate::device::goxlr::device_config::GoXLRDeviceConfiguration;
use crate::device::messaging::DeviceMessage;
use crate::servers::http_server::PatchEvent;
use crate::stop::Stop;

struct DeviceManager {
    last_status: DaemonStatus,
    patch_broadcast: Sender<PatchEvent>,

    /// Used for Devices sending messages back to the Manager
    device_receiver: mpsc::Receiver<RunnerMessage>,
    device_sender: mpsc::Sender<RunnerMessage>,

    /// Used for notification that the DeviceStatus needs refreshing..
    update_receiver: mpsc::Receiver<()>,
    update_sender: mpsc::Sender<()>,

    /// List of all currently known devices..
    devices: Vec<USBLocation>,

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
    pub fn new(shutdown: Stop, broadcast_tx: Sender<PatchEvent>) -> Self {
        let (device_sender, device_receiver) = mpsc::channel(128);
        let (update_sender, update_receiver) = mpsc::channel(1);

        Self {
            last_status: DaemonStatus::default(),
            patch_broadcast: broadcast_tx,

            device_receiver,
            device_sender,

            update_sender,
            update_receiver,

            devices: Default::default(),
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
                    if self.handle_command(message).await {
                        self.update_status().await;
                    }
                }
                Some(device) = device_recv.recv() => {
                    match device {
                        PnPDeviceMessage::Attached(device) => {
                            debug!("[DeviceManager] Received Device: {:?}", device);
                            self.devices.push(device.clone());
                            self.add_device(device.clone()).await;
                        }
                        PnPDeviceMessage::Removed(device) => {
                            self.devices.retain(|d| d != &device);
                            debug!("[DeviceManager] Device Removed: {:?}", device);
                            self.remove_device(device).await;
                        },
                    }
                },
                Some(message) = self.device_receiver.recv() => {
                    debug!("[DeviceManager] Received State Change from GoXLR: {:?}", message);
                    match message {
                        RunnerMessage::UpdateState(device, state) => {
                            self.update_state(device, state).await;
                        }
                        RunnerMessage::Error(device) => {
                            self.handle_error(device);
                        },
                    }
                },
                Some(()) = self.update_receiver.recv() => {
                    self.update_status().await;
                }
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
                            self.update_state(device, state).await;
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
        debug!("[DeviceManager]{} New Device added..", device);

        if !self.devices.contains(&device) {
            debug!("Device Not Known, aborting..");
            return;
        }

        let stop = Stop::new();
        let location = device.clone();
        let (manager_send, manager_recv) = mpsc::channel(64);

        // Ok, we have a new device, we need to add it and set it up..
        let config = GoXLRDeviceConfiguration {
            stop: stop.clone(),
            device,
            update_sender: self.update_sender.clone(),
            manager_sender: self.device_sender.clone(),
            manager_recv,
        };

        let state = DeviceState {
            stop,
            state: RunnerState::Starting,
            messanger: manager_send,
        };

        self.states.insert(location, state);
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

        debug!("Updating DaemonStatus due to device removal");
        self.update_status().await;
    }

    async fn check_devices(&mut self) {
        let mut refresh = vec![];

        // We need to see if any of our devices are in an error state, if so, reset them..
        for (location, state) in &mut self.states {
            if let RunnerState::Error(time) = state.state {
                if let Ok(elapsed) = time.elapsed() {
                    if elapsed.as_secs() >= 2 {
                        debug!(
                            "[DeviceManager]{} Attempting Recovery on Device..",
                            location
                        );
                        refresh.push(location.clone());
                    }
                }
            }
        }

        // Refresh any devices that are in an error state..
        for device in refresh {
            debug!("Handling Device..");
            self.add_device(device).await;
        }
    }

    async fn update_state(&mut self, device: USBLocation, state: RunnerState) {
        if let RunnerState::Running(serial) = &state {
            info!(
                "[DeviceManager]{} Serial {} entered Running State",
                device, serial
            );
            self.serials.insert(serial.to_owned(), device.clone());

            debug!("Device Active, Updating DaemonStatus state..");
            self.update_status().await;
        }

        if let Some(current) = self.states.get_mut(&device) {
            if state == RunnerState::Stopped {
                debug!("[DeviceManager]{} Device Terminated", device);

                // If we get here, the device has stopped, we should clear it..
                self.serials.retain(|_, dev| *dev != device);

                // If we're in a 'Stopping' state, we're prepping for removal..
                if current.state == RunnerState::Stopping || self.stopping {
                    self.states.remove(&device);
                } else {
                    // We've stopped, but we're not supposed to, that's an error.
                    debug!(
                        "[DeviceManager]{} Unexpected Device Stop, attempting recovery",
                        device
                    );
                    current.state = RunnerState::Error(SystemTime::now());
                }

                self.update_status().await;
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
        // Called when somethings gone wrong with the device.. If it's been removed, hopefully
        // the PnP handler has removed it's presence in the 'state' map to prevent awefulness..
        if let Some(current) = self.states.get_mut(&device) {
            // Errors should internally break loops, so we don't need to call stop..
            current.state = RunnerState::Error(SystemTime::now());
        } else {
            debug!("[DeviceManager]{} Device not in state map.", device);
        }

        // If we're tracking a serial for this device, we need to remove it from the list..
        self.serials.retain(|_, dev| *dev != device);
    }

    fn devices_stopped(&self) -> bool {
        for state in self.states.values() {
            let current_state = &state.state;
            match current_state {
                RunnerState::Stopped | RunnerState::Error(_) => {}
                _ => return false,
            }
        }
        true
    }

    async fn update_status(&mut self) {
        let mut status = DaemonStatus::default();

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
                        status.devices.insert(
                            serial.clone(),
                            DeviceStatus {
                                serial: serial.clone(),
                                config: profile,
                            },
                        );
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

        let previous = serde_json::to_value(&self.last_status).unwrap();
        let new = serde_json::to_value(&status).unwrap();

        let patch = diff(&previous, &new);
        if !patch.0.is_empty() {
            // Broadcast Patch..
            let _ = self.patch_broadcast.send(PatchEvent { data: patch });
        }

        self.last_status = status;
    }

    async fn handle_command(&self, command: DeviceMessage) -> bool {
        let mut update = false;

        match command {
            DeviceMessage::GetStatus(tx) => {
                let _ = tx.send(self.last_status.clone());
            }
            DeviceMessage::RunDaemon(_command, tx) => {
                let _ = tx.send(DaemonResponse::Ok);
                update = true;
            }
            DeviceMessage::RunDevice(serial, command, tx) => {
                if let Some(usb) = self.serials.get(&*serial) {
                    if let Some(device) = self.states.get(usb) {
                        let (cmd_tx, cmd_rx) = oneshot::channel();

                        let result = device.messanger.send(Execute(command, cmd_tx)).await;
                        if let Err(e) = result {
                            let _ = tx.send(GoXLRCommandResponse::Error(e.to_string()));
                            return false;
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
                update = true;
            }
        }
        update
    }
}

pub async fn start_device_manager(
    message_receiver: mpsc::Receiver<DeviceMessage>,
    shutdown: Stop,
    broadcast_tx: Sender<PatchEvent>,
) {
    let mut manager = DeviceManager::new(shutdown, broadcast_tx);
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
    Error(SystemTime),
}
