use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use log::{debug, error, info};
use rusb::Error::Pipe;
use rusb::{
    Device, DeviceDescriptor, DeviceHandle, Direction, GlobalContext, Language, Recipient,
    RequestType,
};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::{select, task, time};

use crate::requests::GoXLRMessage;
use crate::runners::device::{DeviceMessage, EventType};
use crate::USBLocation;

pub(crate) struct GoXLRDevice {
    config: GoXLRConfiguration,
    stop: Arc<AtomicBool>,
    task: Option<JoinHandle<()>>,

    handle: DeviceHandle<GlobalContext>,

    device: Device<GlobalContext>,
    descriptor: DeviceDescriptor,

    language: Language,
    timeout: Duration,
    command_count: u16,
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

    pub async fn initialise(&mut self) -> Result<()> {
        // We need to load our GoXLR stuff, init the device, then return..
        debug!("[DEVICE]{} Initialising", self.config.device);

        self.inner_init().await?;

        let messenger = self.config.messenger.clone();
        let device = self.config.device;
        let stop = self.stop.clone();
        let events = self.config.events.clone();
        // Once we're done with that, spawn an event handler..
        self.task = Some(task::spawn(async move {
            debug!("[DEVICE]{} Spawning Event Loop..", device);
            let mut ticker = time::interval(Duration::from_millis(2000));
            loop {
                select! {
                    _ = ticker.tick() => {
                        if stop.load(Ordering::Relaxed) {
                            debug!("[DEVICE]{} Stopping Event Loop..", device);
                            break;
                        }
                        debug!("[DEVICE]{} Event Loop Tick..", device);
                        let (sender, receiver) = oneshot::channel();

                        let _ = messenger.send(GoXLRMessage::GetStatus(sender)).await;
                        match receiver.await {
                            Ok(response) => {
                                // Send the response upstream..
                                let response = DeviceMessage::Event(EventType::Status(response));
                                let _ = events.send(response).await;
                            }
                            Err(error) => {
                                // Something's gone wrong polling, bail.
                                error!("Error in Command Receiver: {}", error);
                                let _ = events.send(DeviceMessage::Error).await;
                                break;
                            },
                        }
                    }
                }
            }
            debug!("[DEVICE]{} Event Loop Stopped", device);
        }));

        Ok(())
    }

    async fn inner_init(&mut self) -> Result<()> {
        // This command 'resets' the GoXLR to a clean state..
        let reset_control = WriteControl {
            request: 1,
            value: 0,
            index: 0,
            data: &[],
        };

        // Attempt to execute it..
        let result = self.write_vendor_control(reset_control);
        if result == Err(Pipe) {
            // The GoXLR is not initialised, we need to fix that..
            info!("Found uninitialised GoXLR, attempting initialisation..");
            self.handle.set_auto_detach_kernel_driver(true)?;

            if self.handle.claim_interface(0).is_err() {
                bail!("Unable to Claim Device");
            }

            debug!("Activating Vendor Interface...");
            self.read_control(ReadControl {
                request: 0,
                value: 0,
                index: 0,
                length: 24,
            })?;

            // Now activate audio..
            debug!("Activating Audio...");
            self.write_class_control(WriteControl {
                request: 1,
                value: 0x0100,
                index: 0x2900,
                data: &[0x80, 0xbb, 0x00, 0x00],
            })?;

            self.handle.release_interface(0)?;

            // Reset the device, so ALSA can pick it up again..
            self.handle.reset()?;

            // Reattempt the reset..
            self.write_vendor_control(reset_control)?;

            // Pause for a second, as we can grab devices a little too quickly!
            sleep(Duration::from_secs(2)).await;
        }

        // Force command pipe activation in all cases.
        debug!("Handling initial request");
        let init = ReadControl {
            request: 3,
            value: 0,
            index: 0,
            length: 1040,
        };
        self.read_control(init)?;
        Ok(())
    }

    // No point making any of these async, as they're limited by libusb
    fn write_vendor_control(&mut self, control: WriteControl<'_>) -> Result<(), rusb::Error> {
        self.write_control(RequestType::Vendor, control)?;
        Ok(())
    }

    fn write_class_control(&mut self, control: WriteControl<'_>) -> Result<(), rusb::Error> {
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

    fn read_control(&mut self, control: ReadControl) -> Result<Vec<u8>, rusb::Error> {
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

    pub async fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);

        // Rejoin on the task, and hold the stop request until we're finished..
        if self.task.is_some() {
            let _ = self.task.take().unwrap().await;
        }
    }
}

#[derive(Clone, Copy)]
struct WriteControl<'a> {
    request: u8,
    value: u16,
    index: u16,
    data: &'a [u8],
}

struct ReadControl {
    request: u8,
    value: u16,
    index: u16,
    length: usize,
}

pub struct GoXLRConfiguration {
    pub(crate) device: USBLocation,
    pub(crate) messenger: mpsc::Sender<GoXLRMessage>,
    pub(crate) events: mpsc::Sender<DeviceMessage>,
}
