use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;
use goxlr_shared::device::DeviceType;
use crate::platform::FullGoXLRDevice;

use crate::runners::device::InternalDeviceMessage;
use crate::USBLocation;

#[derive(Clone)]
pub struct GoXLRConfiguration {
    pub(crate) device: USBLocation,
    pub(crate) events: mpsc::Sender<InternalDeviceMessage>,
}

#[async_trait]
pub trait GoXLRDevice {
    async fn from_config(config: GoXLRConfiguration) -> Result<Box<dyn FullGoXLRDevice>> where Self: Sized;
    async fn run(&mut self) -> anyhow::Result<()>;
    async fn stop(&mut self);
    fn get_device_type(&self) -> DeviceType;
}