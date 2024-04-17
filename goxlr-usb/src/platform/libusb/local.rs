use crate::platform::libusb::device::LibUSBGoXLR;
use crate::USBLocation;
use anyhow::{bail, Result};
use rusb::{Device, DeviceDescriptor, Direction, GlobalContext, Recipient, RequestType};

impl LibUSBGoXLR {
    /// Gets a Device Handle from libUSB
    pub(crate) fn find_device(
        goxlr: USBLocation,
    ) -> Result<(Device<GlobalContext>, DeviceDescriptor)> {
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
