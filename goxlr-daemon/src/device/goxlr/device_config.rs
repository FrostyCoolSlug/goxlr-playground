use tokio::sync::mpsc::{Receiver, Sender};

use goxlr_usb::USBLocation;

use crate::device::device_manager::{ManagerMessage, RunnerMessage};
use crate::stop::Stop;

pub struct GoXLRDeviceConfiguration {
    pub(crate) stop: Stop,
    pub(crate) device: USBLocation,
    pub(crate) update_sender: Sender<()>,
    pub(crate) manager_sender: Sender<RunnerMessage>,
    pub(crate) manager_recv: Receiver<ManagerMessage>,
}
