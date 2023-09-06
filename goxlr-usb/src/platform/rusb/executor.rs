use std::time::Duration;

use anyhow::bail;
use anyhow::Result;
use async_trait::async_trait;
use byteorder::{ByteOrder, LittleEndian};
use log::debug;
use rusb::Error::Pipe;
use tokio::time::sleep;

use crate::common::executor::ExecutableGoXLR;
use crate::goxlr::commands::Command;
use crate::platform::rusb::device::{GoXLRDevice, ReadControl, WriteControl};
use crate::PID_GOXLR_MINI;

/**
    We're going to handle this slightly differently from the original GoXLR utility, the main
    problem previously was that handling errors (such as index desyncs) was problematic as it
    required an entire retry mechanism built into perform_request, given that we have a method
    elsewhere who's entire purpose is to call perform_request(_,_, false), it figures we should
    just handle attempts to recover from failure there!

    This approach ultimately makes this code a lot simpler..
*/

#[async_trait]
impl ExecutableGoXLR for GoXLRDevice {
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

        let control = WriteControl {
            request: 2,
            value: 0,
            index: 0,
            data: &full_request,
        };

        if let Err(error) = self.write_vendor_control(control) {
            debug!("Error when attempting to write control.");
            bail!(error);
        }

        // The mini is a little slower than the full device, set poll times to reflect that.
        let sleep_time = if self.descriptor.product_id() == PID_GOXLR_MINI {
            Duration::from_millis(10)
        } else {
            Duration::from_millis(3)
        };
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
                    bail!("Error Reading GoXLR (Timeout): {:?}", response_value.err());
                }
            }
            if response_value.is_err() {
                let err = response_value.err().unwrap();
                bail!("Error Reading Response from GoXLR: {:?}", err);
            }

            let mut response_header = response_value.unwrap();
            if response_header.len() < 16 {
                let len = response_header.len();
                bail!("Invalid Response Length from GoXLR, Count {}", len);
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

                bail!("Response doesn't match request");
            }

            debug_assert!(response.len() == response_length as usize);
            break;
        }

        Ok(response)
    }

    async fn perform_recovery(&mut self) -> Result<()> {
        todo!()
    }

    async fn perform_stop(&mut self) {
        self.stop().await
    }
}
