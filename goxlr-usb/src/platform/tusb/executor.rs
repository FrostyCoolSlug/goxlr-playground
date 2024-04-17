use anyhow::{Result, bail};
use async_trait::async_trait;
use byteorder::{ByteOrder, LittleEndian};
use log::{debug, error};
use crate::common::executor::ExecutableGoXLR;
use crate::goxlr::commands::Command;
use crate::platform::common::device::GoXLRDevice;
use crate::platform::tusb::device::TUSBAudioGoXLR;

#[async_trait]
impl ExecutableGoXLR for TUSBAudioGoXLR {
    async fn perform_request(&mut self, command: Command, body: &[u8]) -> Result<Vec<u8>> {
        if command == Command::ResetCommandIndex {
            self.command_count = 0;
        } else {
            if self.command_count == u16::MAX {
                let _ = self.request_data(Command::ResetCommandIndex, &[]).await?;
            }
            self.command_count += 1;
        }

        let command_index = self.command_count;
        let mut full_request = vec![0; 16];
        LittleEndian::write_u32(&mut full_request[0..4], command.command_id());
        LittleEndian::write_u16(&mut full_request[4..6], body.len() as u16);
        LittleEndian::write_u16(&mut full_request[6..8], command_index);
        full_request.extend(body);

        if let Err(error) = self.write_control(2, 0, 0, &full_request) {
            bail!("Error Sending Message: {}", error);
        }

        // We will sit here, and wait for a response.. this may take a few cycles..
        if !self.await_data().await {
            bail!("Event handler has ended, Disconnecting.");
        }

        let mut response_value = self.read_control(3, 0, 0, 1040);
        if let Err(error) = response_value {
            bail!("Error Reading Response: {:?}", error);
        }

        let mut response_header = response_value?;
        if response_header.len() < 16 {
            error!(
                "Invalid Response received from the GoXLR, Expected: 16, Received: {}",
                response_header.len()
            );
            bail!("Invalid Response");
        }

        let response = response_header.split_off(16);
        let response_length = LittleEndian::read_u16(&response_header[4..6]);
        let response_command_index = LittleEndian::read_u16(&response_header[6..8]);

        if response_command_index != command_index {
            bail!("Command Index Mismatch, Expected: {}, Received: {}", command_index, response_command_index);
        }

        debug_assert!(response.len() == response_length as usize);
        Ok(response)
    }

    async fn perform_recovery(&mut self) -> Result<()> {
        bail!("Not Implemented!");
    }

    async fn perform_stop(&mut self) {
        self.stop().await
    }
}
