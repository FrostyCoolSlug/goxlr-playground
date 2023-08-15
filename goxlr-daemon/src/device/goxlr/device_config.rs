use tokio::sync::mpsc::Sender;

use goxlr_usb_messaging::USBLocation;

use crate::device::device_manager::RunnerMessage;
use crate::stop::Stop;

pub struct GoXLRDeviceConfiguration {
    pub(crate) stop: Stop,
    pub(crate) device: USBLocation,
    pub(crate) manager_sender: Sender<RunnerMessage>,
}
