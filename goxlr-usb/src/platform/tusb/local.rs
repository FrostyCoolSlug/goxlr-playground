use anyhow::Result;

use crate::platform::tusb::device::TUSBAudioGoXLR;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::error::TryRecvError;

/// This class is for local methods in the TUSB handler, such as reading / writing controls
impl TUSBAudioGoXLR {
    pub(crate) fn write_control(
        &self,
        request: u8,
        value: u16,
        index: u16,
        data: &[u8],
    ) -> Result<()> {
        self.handle.send_request(request, value, index, data)
    }

    pub(crate) fn read_control(
        &mut self,
        request: u8,
        value: u16,
        index: u16,
        length: usize,
    ) -> Result<Vec<u8>> {
        self.handle.read_response(request, value, index, length)
    }

    pub(crate) async fn await_data(&mut self) -> bool {
        // This is a crap load easier now, just wait for the event callback..
        if let Some(receiver) = &mut self.device_data_received {
            if let Some(result) = receiver.recv().await {
                result
            } else {
                false
            }
        } else {
            false
        }
    }
}
