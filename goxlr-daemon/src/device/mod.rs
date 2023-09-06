use tokio::sync::mpsc::Sender;

use crate::device::messaging::DeviceCommand;

pub mod device_manager;
pub(crate) mod goxlr;
mod messaging;

pub mod packet;
