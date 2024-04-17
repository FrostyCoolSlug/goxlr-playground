use std::sync::atomic::Ordering;
use anyhow::bail;
use log::debug;
use crate::platform::common::device::GoXLRDevice;
use crate::platform::common::initialiser::InitialisableGoXLR;
use crate::platform::tusb::device::TUSBAudioGoXLR;

impl InitialisableGoXLR for TUSBAudioGoXLR {
    async fn initialise(&mut self) -> anyhow::Result<()> {
        // Perform Device Setup..
        // Activate the Vendor interface, also initialises audio on Windows!
        if let Err(error) = self.handle.read_response(0, 0, 0, 24) {
            self.stop().await;
            bail!("Error Reading Initial Packet: {}", error);
        }

        // Perform soft reset.
        if let Err(error) = self.handle.send_request(1, 0, 0, &[]) {
            self.stop().await;
            bail!("Error Sending initial Reset Packet: {}", error);
        }

        // Wait for the response event, then read..
        if !self.await_data().await {
            bail!("Error received from Event Handler..");
        }

        if let Err(error) = self.handle.read_response(3, 0, 0, 1040) {
            self.stop().await;
            bail!("Error Reading Response to Initial Reset: {}", error);
        }

        Ok(())
    }
}
