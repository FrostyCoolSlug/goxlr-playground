use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use goxlr_shared::device::DeviceType;
use log::{debug, info, trace};
use rusb::{
    Device, DeviceDescriptor, DeviceHandle, Direction, GlobalContext, Language, Recipient,
    RequestType,
};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::{select, task, time};

use crate::common::executor::InitialisableGoXLR;
use crate::runners::device::{DeviceMessage, EventType};
use crate::{USBLocation, PID_GOXLR_MINI};

pub(crate) struct GoXLRDevice {
    config: GoXLRConfiguration,
    stop: Arc<AtomicBool>,
    task: Option<JoinHandle<()>>,

    pub(crate) handle: DeviceHandle<GlobalContext>,
    pub(crate) device: Device<GlobalContext>,
    pub(crate) descriptor: DeviceDescriptor,

    language: Language,
    timeout: Duration,
    pub(crate) command_count: u16,
}

impl GoXLRDevice {
    pub async fn from(config: GoXLRConfiguration) -> Result<Self> {
        let (device, descriptor) = Self::find_device(config.device)?;
        let handle = device.open()?;

        let device = handle.device();

        info!("Connected to possible GoXLR device at {:?}", device);

        let timeout = Duration::from_secs(1);
        let languages = handle.read_languages(timeout)?;
        let language = languages
            .get(0)
            .ok_or_else(|| anyhow!("Not GoXLR?"))?
            .to_owned();

        Ok(Self {
            config,
            stop: Arc::new(AtomicBool::new(false)),
            task: None,

            handle,
            device,
            descriptor,
            language,
            timeout,
            command_count: 0,
        })
    }

    fn find_device(goxlr: USBLocation) -> Result<(Device<GlobalContext>, DeviceDescriptor)> {
        if let Ok(devices) = rusb::devices() {
            for usb_device in devices.iter() {
                if usb_device.bus_number() == goxlr.bus_number
                    && usb_device.address() == goxlr.address
                {
                    if let Ok(descriptor) = usb_device.device_descriptor() {
                        return Ok((usb_device, descriptor));
                    }
                }
            }
        }
        bail!("Specified Device not Found!")
    }

    pub async fn run(&mut self) -> Result<()> {
        let poll_millis = 20;

        // We need to load our GoXLR stuff, init the device, then return..
        debug!("[DEVICE]{} Initialising", self.config.device);

        self.initialise().await?;

        let device = self.config.device;
        let stop = self.stop.clone();
        let events = self.config.events.clone();
        // Once we're done with that, spawn an event handler..
        self.task = Some(task::spawn(async move {
            debug!("[DEVICE]{} Spawning Event Loop..", device);
            let mut ticker = time::interval(Duration::from_millis(poll_millis));
            loop {
                select! {
                    _ = ticker.tick() => {
                        // Under Linux we're not able to listen to INTERRUPTs from the USB Device
                        // which would normally indicate the device 'state' has changed.

                        // So we simply use this ticker and tell the parent to do the check.
                        if stop.load(Ordering::Relaxed) {
                            debug!("[DEVICE]{} Stopping Event Loop..", device);
                            break;
                        }

                        trace!("[DEVICE]{} Event Loop Tick..", device);
                        let _ = events.send(DeviceMessage::Event(EventType::StatusChange)).await;
                    }
                }
            }
            debug!("[DEVICE]{} Event Loop Stopped", device);
        }));

        Ok(())
    }

    // No point making any of these async, as they're limited by libusb
    pub(crate) fn write_vendor_control(
        &mut self,
        control: WriteControl<'_>,
    ) -> Result<(), rusb::Error> {
        self.write_control(RequestType::Vendor, control)?;
        Ok(())
    }

    pub(crate) fn write_class_control(
        &mut self,
        control: WriteControl<'_>,
    ) -> Result<(), rusb::Error> {
        self.write_control(RequestType::Class, control)?;
        Ok(())
    }

    fn write_control(
        &mut self,
        request_type: RequestType,
        control: WriteControl<'_>,
    ) -> Result<(), rusb::Error> {
        self.handle.write_control(
            rusb::request_type(Direction::Out, request_type, Recipient::Interface),
            control.request,
            control.value,
            control.index,
            control.data,
            self.timeout,
        )?;

        Ok(())
    }

    pub(crate) fn read_control(&mut self, control: ReadControl) -> Result<Vec<u8>, rusb::Error> {
        let mut buf = vec![0; control.length];
        let response_length = self.handle.read_control(
            rusb::request_type(Direction::In, RequestType::Vendor, Recipient::Interface),
            control.request,
            control.value,
            control.index,
            &mut buf,
            self.timeout,
        )?;
        buf.truncate(response_length);
        Ok(buf)
    }

    pub(crate) fn get_device_type(&self) -> DeviceType {
        if self.descriptor.product_id() == PID_GOXLR_MINI {
            return DeviceType::Mini;
        }
        DeviceType::Full
    }

    pub async fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);

        // Rejoin on the task, and hold the stop request until we're finished..
        if self.task.is_some() {
            let _ = self.task.take().unwrap().await;
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct WriteControl<'a> {
    pub(crate) request: u8,
    pub(crate) value: u16,
    pub(crate) index: u16,
    pub(crate) data: &'a [u8],
}

#[derive(Clone, Copy)]
pub(crate) struct ReadControl {
    pub(crate) request: u8,
    pub(crate) value: u16,
    pub(crate) index: u16,
    pub(crate) length: usize,
}

pub struct GoXLRConfiguration {
    pub(crate) device: USBLocation,
    pub(crate) events: mpsc::Sender<DeviceMessage>,
}
