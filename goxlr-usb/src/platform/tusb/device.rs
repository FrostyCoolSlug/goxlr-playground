use anyhow::Result;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use anyhow::{anyhow, bail};
use async_trait::async_trait;
use log::{debug, error, info};
use rusb::{Device, DeviceDescriptor, Direction, GlobalContext, Language, Recipient, RequestType};
use tokio::{join, select, task, time};
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Receiver;
use goxlr_shared::device::DeviceType;
use crate::{PID_GOXLR_MINI, USBLocation};
use crate::common::command_handler::GoXLRCommands;
use crate::platform::common::device::{GoXLRConfiguration, GoXLRDevice};
use crate::platform::common::initialiser::InitialisableGoXLR;
use crate::platform::FullGoXLRDevice;
use crate::platform::tusb::tusbaudio::{DeviceHandle, EventChannelReceiver, EventChannelSender, TUSB_INTERFACE};
use crate::runners::device::InternalDeviceMessage;
use crate::util::stop::Stop;

pub(crate) struct TUSBAudioGoXLR {
    config: GoXLRConfiguration,
    stop: Stop,
    runner: Option<task::JoinHandle<()>>,

    device_runner: Option<thread::JoinHandle<()>>,
    device_runner_stop: Arc<AtomicBool>,

    pub(crate) handle: DeviceHandle,
    pub(crate) device_data_received: Option<Receiver<bool>>,
    pub(crate) command_count: u16,
}

#[async_trait]
impl GoXLRDevice for TUSBAudioGoXLR {
    async fn from_config(config: GoXLRConfiguration) -> Result<Box<dyn FullGoXLRDevice>> where Self: Sized {
        // Similarly to the libUSB version, we grab a handle to the GoXLR here which will be used
        // going forwards.
        let handle = if let Some(win_usb) = &config.device.windows_usb {
            DeviceHandle::from_identifier(win_usb.identifier.clone())?
        } else {
            bail!("Unable to Locate Device");
        };

        Ok(Box::new(TUSBAudioGoXLR {
            config,
            stop: Stop::new(),

            runner: None,

            device_runner: None,
            device_runner_stop: Arc::new(AtomicBool::new(false)),

            handle,
            device_data_received: None,
            command_count: 0,
        }))
    }

    async fn run(&mut self) -> Result<()> {
        // The world is a little different here, all our main task needs to do is listen for
        // event triggers from the GoXLR, and fire off a Poll message when one is received..
        let identifier = if let Some(win_usb) = &self.config.device.windows_usb {
            win_usb.identifier.clone()
        } else {
            bail!("USB Device Identifier Not Defined!")
        };


        let device = self.config.device.clone();
        let mut stop = self.stop.clone();
        let events = self.config.events.clone();

        // Firstly, a callback to trigger once all the Callback handlers are created
        let (ready_sender, ready_recv) = oneshot::channel();

        // Next, this event triggers once a command response is ready
        let (data_sender, data_recv) = mpsc::channel(1);
        self.device_data_received = Some(data_recv);

        // Next, this queue handles device input events
        let (event_sender, mut event_recv) = mpsc::channel(1);

        // Clone the Stopper..
        let device_stop = self.device_runner_stop.clone();

        // Spawn up the TUSB Handler..
        self.device_runner = Some(thread::spawn(move || {
            let sender = EventChannelSender {
                ready_notifier: ready_sender,
                data_read: data_sender,
                device_event: event_sender,
            };

            let _ = TUSB_INTERFACE.event_loop(
                identifier,
                sender,
                device_stop
            );
        }));

        // Wait for the Handler to be ready...
        if let Err(e) = ready_recv.await {
            self.stop().await;
            bail!("Error Spawning TUSB Handler: {}", e);
        }

        self.initialise().await?;
        
        // Event Sender..
        let internal_sender = self.config.events.clone();

        // Once we're done with that, spawn an event handler..
        self.runner = Some(task::spawn(async move {
            debug!("[DEVICE]{} Spawning Event Loop..", device);
            loop {
                select! {
                    Some(()) = event_recv.recv() => {
                        debug!("[DEVICE]{} Event Notification Received..", device);
                        let _ = internal_sender.send(InternalDeviceMessage::Poll).await;
                    }
                    _ = stop.recv() => {
                        debug!("[DEVICE]{} Stopping Event Loop..", device);
                        break;
                    }
                }
            }
            debug!("[DEVICE]{} Event Loop Stopped", device);
        }));

        Ok(())
    }

    async fn stop(&mut self) {
        self.stop.trigger();
        self.device_runner_stop.store(true, Ordering::Relaxed);

        debug!("Waiting for Threads to End..");
        // Rejoin on the task, and hold the stop request until we're finished...
        if let Some(runner) = self.runner.take() {
            let _ = runner.await;
            debug!("Main Runner Ended")
        }

        if let Some(runner) = self.device_runner.take() {
            let _ = runner.join();
            debug!("Device Runner Ended")
        }
    }

    fn get_device_type(&self) -> DeviceType {
        // TODO: Fix this..
        return DeviceType::Full;
    }
}



impl GoXLRCommands for TUSBAudioGoXLR {}
impl FullGoXLRDevice for TUSBAudioGoXLR {}

