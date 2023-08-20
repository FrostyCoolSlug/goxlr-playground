use std::time::Duration;

use anyhow::bail;
use async_trait::async_trait;
use byteorder::{ByteOrder, LittleEndian};
use log::{debug, error, warn};
use rusb::Error::Pipe;
use tokio::time::sleep;

use crate::common::executor::ExecutableGoXLR;
use crate::goxlr::commands::Command;
use crate::platform::rusb::device::{GoXLRDevice, ReadControl, WriteControl};

#[async_trait]
impl ExecutableGoXLR for GoXLRDevice {
    async fn perform_request(
        &mut self,
        command: Command,
        body: &[u8],
        retry: bool,
    ) -> anyhow::Result<Vec<u8>> {
        if command == Command::ResetCommandIndex {
            self.command_count = 0;
        } else {
            if self.command_count == u16::MAX {
                let result = self.request_data(Command::ResetCommandIndex, &[]).await;
                if result.is_err() {
                    self.stop().await;
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

        let control = WriteControl {
            request: 2,
            value: 0,
            index: 0,
            data: &full_request,
        };

        if let Err(error) = self.write_vendor_control(control) {
            debug!("Error when attempting to write control.");

            self.stop().await;
            bail!(error);
        }

        // The full fat GoXLR can handle requests incredibly quickly..
        let mut sleep_time = Duration::from_millis(3);
        // if self.descriptor.product_id() == PID_GOXLR_MINI {
        //     // The mini, however, cannot.
        //     sleep_time = Duration::from_millis(10);
        // }
        sleep(sleep_time).await;

        let read_control = ReadControl {
            request: 3,
            value: 0,
            index: 0,
            length: 1040,
        };

        let mut response = vec![];
        for i in 0..20 {
            let response_value = self.read_control(read_control);
            if response_value == Err(Pipe) {
                if i < 19 {
                    debug!("Response not arrived yet for {:?}, sleeping and retrying (Attempt {} of 20)", command, i + 1);
                    sleep(sleep_time).await;
                    continue;
                } else {
                    // We can't read from this GoXLR, flag as disconnected.
                    warn!("Failed to receive response (Attempt 20 of 20), possible Dead GoXLR?");

                    self.stop().await;
                    bail!("Error Reading GoXLR: {:?}", response_value.err());
                }
            }
            if response_value.is_err() {
                let err = response_value.err().unwrap();
                debug!("Error Occurred during packet read: {}", err);

                self.stop().await;
                bail!("Error Reading Response from GoXLR: {:?}", err);
            }

            let mut response_header = response_value.unwrap();
            if response_header.len() < 16 {
                let len = response_header.len();
                error!(
                    "Invalid Response received from the GoXLR, Expected: 16, Received: {}",
                    len
                );

                self.stop().await;
                bail!(
                    "Invalid Response from GoXLR, Expected len > 16, Received {}",
                    len
                );
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
                        self.stop().await;
                        return result;
                    }

                    debug!("Resync complete, retrying Command..");
                    let result = self.perform_request(command, body, true).await;
                    return result;
                } else {
                    debug!("Resync Failed, Throwing Error..");

                    self.stop().await;
                    bail!("Critical Sync error with GoXLR");
                };
            }

            debug_assert!(response.len() == response_length as usize);
            break;
        }

        Ok(response)
    }
}
