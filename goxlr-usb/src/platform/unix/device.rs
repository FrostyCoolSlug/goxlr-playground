use crate::commands::Command;
use crate::platform::base::{
    AttachGoXLR, ExecutableGoXLR, FullGoXLRDevice, GoXLRCommands, UsbData,
};
use crate::state_tracker::{ChangeEvent, GoXLRStateTracker};
use crate::{GoXLRDevice, PID_GOXLR_MINI};
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use byteorder::{ByteOrder, LittleEndian};
use log::{debug, error, info, warn};
use rusb::Error::Pipe;
use rusb::{
    Device, DeviceDescriptor, DeviceHandle, Direction, GlobalContext, Language, Recipient,
    RequestType,
};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::time;
use tokio::time::sleep;

pub(crate) struct GoXLRUSB {
    goxlr: GoXLRDevice,
    handle: DeviceHandle<GlobalContext>,
    device: Device<GlobalContext>,
    descriptor: DeviceDescriptor,

    language: Language,
    timeout: Duration,
    command_count: u16,
}

impl AttachGoXLR for GoXLRUSB {}

#[async_trait]
impl ExecutableGoXLR for GoXLRUSB {
    async fn perform_request(
        &mut self,
        command: Command,
        body: &[u8],
        retry: bool,
    ) -> Result<Vec<u8>> {
        if command == Command::ResetCommandIndex {
            self.command_count = 0;
        } else {
            if self.command_count == u16::MAX {
                let result = self.request_data(Command::ResetCommandIndex, &[]).await;
                if result.is_err() {
                    return result;
                }
            }
            self.command_count += 1;
        }

        let command_index = self.command_count;
        let mut full_request = vec![0; 16];
        LittleEndian::write_u32(&mut full_request[0..4], command.command_id());
        LittleEndian::write_u16(&mut full_request[4..6], body.len() as u16);
        LittleEndian::write_u16(&mut full_request[6..8], command_index);
        full_request.extend(body);

        if let Err(error) = self.write_control(2, 0, 0, &full_request).await {
            debug!("Error when attempting to write control.");
            bail!(error);
        }

        // The full fat GoXLR can handle requests incredibly quickly..
        let mut sleep_time = Duration::from_millis(3);
        if self.descriptor.product_id() == PID_GOXLR_MINI {
            // The mini, however, cannot.
            sleep_time = Duration::from_millis(10);
        }
        sleep(sleep_time).await;

        let mut response = vec![];
        for i in 0..20 {
            let response_value = self.read_control(3, 0, 0, 1040).await;
            if response_value == Err(Pipe) {
                if i < 19 {
                    debug!("Response not arrived yet for {:?}, sleeping and retrying (Attempt {} of 20)", command, i + 1);
                    sleep(sleep_time).await;
                    continue;
                } else {
                    // We can't read from this GoXLR, flag as disconnected.
                    warn!("Failed to receive response (Attempt 20 of 20), possible Dead GoXLR?");
                    bail!(Pipe)
                }
            }
            if response_value.is_err() {
                let err = response_value.err().unwrap();
                debug!("Error Occurred during packet read: {}", err);
                bail!(err);
            }

            let mut response_header = response_value.unwrap();
            if response_header.len() < 16 {
                error!(
                    "Invalid Response received from the GoXLR, Expected: 16, Received: {}",
                    response_header.len()
                );
                bail!(Pipe);
            }

            response = response_header.split_off(16);
            let response_length = LittleEndian::read_u16(&response_header[4..6]);
            let response_command_index = LittleEndian::read_u16(&response_header[6..8]);

            if response_command_index != command_index {
                debug!("Mismatched Command Indexes..");
                debug!(
                    "Expected {}, received: {}",
                    command_index, response_command_index
                );
                debug!("Full Request: {:?}", full_request);
                debug!("Response Header: {:?}", response_header);
                debug!("Response Body: {:?}", response);

                return if !retry {
                    debug!("Attempting Resync and Retry");
                    let result = self
                        .perform_request(Command::ResetCommandIndex, &[], true)
                        .await;
                    if result.is_err() {
                        return result;
                    }

                    debug!("Resync complete, retrying Command..");
                    let result = self.perform_request(command, body, true).await;
                    return result;
                } else {
                    debug!("Resync Failed, Throwing Error..");
                    bail!(rusb::Error::Other);
                };
            }

            debug_assert!(response.len() == response_length as usize);
            break;
        }

        Ok(response)
    }

    async fn get_descriptor(&self) -> Result<UsbData> {
        let version = self.descriptor.usb_version();
        let usb_version = (version.0, version.1, version.2);

        let device_manufacturer = self.handle.read_manufacturer_string(
            self.language,
            &self.descriptor,
            Duration::from_millis(100),
        )?;

        let product_name = self.handle.read_product_string(
            self.language,
            &self.descriptor,
            Duration::from_millis(100),
        )?;

        Ok(UsbData {
            vendor_id: self.descriptor.vendor_id(),
            product_id: self.descriptor.product_id(),
            device_version: usb_version,
            device_manufacturer,
            product_name,
        })
    }

    async fn initialise(&mut self) -> Result<()> {
        // Resets the state of the device (unconfirmed - Might just be the command id counter)
        let result = self.write_control(1, 0, 0, &[]).await;

        if result == Err(Pipe) {
            // The GoXLR is not initialised, we need to fix that..
            info!("Found uninitialised GoXLR, attempting initialisation..");
            self.handle.set_auto_detach_kernel_driver(true)?;

            if self.handle.claim_interface(0).is_err() {
                return Err(anyhow!("Unable to Claim Device"));
            }

            debug!("Activating Vendor Interface...");
            self.read_control(0, 0, 0, 24).await?;

            // Now activate audio..
            debug!("Activating Audio...");
            self.write_class_control(1, 0x0100, 0x2900, &[0x80, 0xbb, 0x00, 0x00])
                .await?;
            self.handle.release_interface(0)?;

            // Reset the device, so ALSA can pick it up again..
            self.handle.reset()?;

            // Reattempt the reset..
            self.write_control(1, 0, 0, &[]).await?;

            warn!(
                "Initialisation complete. If you are using the JACK script, you may need to reboot for audio to work."
            );

            // Pause for a second, as we can grab devices a little too quickly!

            sleep(Duration::from_secs(2)).await;
        }

        // Force command pipe activation in all cases.
        debug!("Handling initial request");
        self.read_control(3, 0, 0, 1040).await?;
        Ok(())
    }
}

impl GoXLRCommands for GoXLRUSB {}
impl FullGoXLRDevice for GoXLRUSB {}

impl GoXLRUSB {
    fn find_device(goxlr: GoXLRDevice) -> Result<(Device<GlobalContext>, DeviceDescriptor)> {
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

    pub(crate) async fn from_device(goxlr: GoXLRDevice) -> Result<Box<(dyn FullGoXLRDevice)>> {
        let (device, descriptor) = GoXLRUSB::find_device(goxlr.clone())?;
        let handle = device.open()?;

        let device = handle.device();

        info!("Connected to possible GoXLR device at {:?}", device);

        let timeout = Duration::from_secs(1);
        let languages = handle.read_languages(timeout)?;
        let language = languages
            .get(0)
            .ok_or_else(|| anyhow!("Not GoXLR?"))?
            .to_owned();

        Ok(Box::new(Self {
            goxlr,
            handle,
            device,
            descriptor,
            language,
            timeout,
            command_count: 0,
        }))
    }

    /// Called to run and handle device related tasks, including the first initialisation, status
    /// polling, and event management.
    async fn spawn_event_handler(&mut self) {
        debug!("Spinning up Linux event handler..");
    }

    pub(crate) async fn write_class_control(
        &mut self,
        request: u8,
        value: u16,
        index: u16,
        data: &[u8],
    ) -> Result<(), rusb::Error> {
        self.handle.write_control(
            rusb::request_type(Direction::Out, RequestType::Class, Recipient::Interface),
            request,
            value,
            index,
            data,
            self.timeout,
        )?;

        Ok(())
    }

    pub(crate) async fn write_control(
        &mut self,
        request: u8,
        value: u16,
        index: u16,
        data: &[u8],
    ) -> Result<(), rusb::Error> {
        self.handle.write_control(
            rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Interface),
            request,
            value,
            index,
            data,
            self.timeout,
        )?;

        Ok(())
    }

    pub(crate) async fn read_control(
        &mut self,
        request: u8,
        value: u16,
        index: u16,
        length: usize,
    ) -> Result<Vec<u8>, rusb::Error> {
        let mut buf = vec![0; length];
        let response_length = self.handle.read_control(
            rusb::request_type(Direction::In, RequestType::Vendor, Recipient::Interface),
            request,
            value,
            index,
            &mut buf,
            self.timeout,
        )?;
        buf.truncate(response_length);
        Ok(buf)
    }
}
