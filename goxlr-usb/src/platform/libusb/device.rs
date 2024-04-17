use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use goxlr_shared::device::DeviceType;
use log::{debug, info};
use rusb::{
    Device, DeviceDescriptor, DeviceHandle, Direction, GlobalContext, Language, Recipient,
    RequestType,
};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::{select, task, time};

use crate::runners::device::InternalDeviceMessage;
use crate::{USBLocation, PID_GOXLR_MINI};
use crate::platform::common::device::{GoXLRConfiguration, GoXLRDevice};
use crate::platform::common::initialiser::InitialisableGoXLR;
use crate::util::stop::Stop;

pub(crate) struct GoXLRDevice {
    config: GoXLRConfiguration,
    stop: Stop,
    task: Option<JoinHandle<()>>,

    pub(crate) handle: DeviceHandle<GlobalContext>,
    pub(crate) device: Device<GlobalContext>,
    pub(crate) descriptor: DeviceDescriptor,

    timeout: Duration,
    pub(crate) command_count: u16,
}

impl GoXLRDevice {
    async fn from(config: GoXLRConfiguration) -> Result<Self> {
        let (device, descriptor) = GoXLRDevice::find_device(config.device.clone())?;
        let handle = device.open()?;
        let device = handle.device();

        info!("Connected to possible GoXLR device at {:?}", device);
        let timeout = Duration::from_secs(1);

        Ok(GoXLRDevice {
            config,
            stop: Stop::new(),
            task: None,

            handle,
            device,
            descriptor,
            timeout,
            command_count: 0,
        })
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

impl GoXLRDevice {
    /// Gets a Device Handle from libUSB
    fn find_device(goxlr: USBLocation) -> Result<(Device<GlobalContext>, DeviceDescriptor)> {
        if let Ok(devices) = rusb::devices() {
            for usb_device in devices.iter() {
                if let Some(lib_usb) = &goxlr.lib_usb {
                    if usb_device.bus_number() == lib_usb.bus_number
                        && usb_device.address() == lib_usb.address
                    {
                        if let Ok(descriptor) = usb_device.device_descriptor() {
                            return Ok((usb_device, descriptor));
                        }
                    }
                }
            }
        }
        bail!("Specified Device not Found!")
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


