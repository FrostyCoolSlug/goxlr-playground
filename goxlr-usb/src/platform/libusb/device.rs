use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use goxlr_shared::device::DeviceType;
use log::{debug, info};
use rusb::{Device, DeviceDescriptor, DeviceHandle, GlobalContext};
use tokio::task::JoinHandle;
use tokio::{select, task, time};

use crate::common::command_handler::GoXLRCommands;
use crate::platform::common::device::{GoXLRConfiguration, GoXLRDevice};
use crate::platform::common::initialiser::InitialisableGoXLR;
use crate::platform::FullGoXLRDevice;
use crate::runners::device::InternalDeviceMessage;
use crate::util::stop::Stop;
use crate::PID_GOXLR_MINI;

pub(crate) struct LibUSBGoXLR {
    config: GoXLRConfiguration,
    stop: Stop,
    task: Option<JoinHandle<()>>,

    pub(crate) handle: DeviceHandle<GlobalContext>,
    pub(crate) device: Device<GlobalContext>,
    pub(crate) descriptor: DeviceDescriptor,

    pub(crate) timeout: Duration,
    pub(crate) command_count: u16,
}

#[async_trait]
impl GoXLRDevice for LibUSBGoXLR {
    async fn from_config(config: GoXLRConfiguration) -> Result<Box<dyn FullGoXLRDevice>>
    where
        Self: Sized,
    {
        let (device, descriptor) = LibUSBGoXLR::find_device(config.device.clone())?;
        let handle = device.open()?;
        let device = handle.device();

        info!("Connected to possible GoXLR device at {:?}", device);
        let timeout = Duration::from_secs(1);

        Ok(Box::new(LibUSBGoXLR {
            config,
            stop: Stop::new(),
            task: None,

            handle,
            device,
            descriptor,
            timeout,
            command_count: 0,
        }))
    }

    async fn run(&mut self) -> Result<()> {
        let poll_millis = 20;

        // We need to load our GoXLR stuff, init the device, then return..
        debug!("[DEVICE]{} Initialising", self.config.device);

        self.initialise().await?;

        let device = self.config.device.clone();
        let events = self.config.events.clone();
        // Once we're done with that, spawn an event handler..

        let mut stop = self.stop.clone();
        self.task = Some(task::spawn(async move {
            debug!("[DEVICE]{} Spawning Event Loop..", device);
            let mut ticker = time::interval(Duration::from_millis(poll_millis));
            loop {
                select! {
                    _ = ticker.tick() => {
                        // Using RUSB, we simple poll on the tick because we don't have access
                        // to Driver Event Messages.. Make sure we only ever have 1 of these
                        // queued up for processing at once.
                        if events.capacity() > 0 {
                            let _ = events.send(InternalDeviceMessage::Poll).await;
                        }
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

        // Rejoin on the task, and hold the stop request until we're finished..
        if self.task.is_some() {
            let _ = self.task.take().unwrap().await;
        }
    }

    fn get_device_type(&self) -> DeviceType {
        if self.descriptor.product_id() == PID_GOXLR_MINI {
            return DeviceType::Mini;
        }
        DeviceType::Full
    }
}

impl GoXLRCommands for LibUSBGoXLR {}
impl FullGoXLRDevice for LibUSBGoXLR {}
