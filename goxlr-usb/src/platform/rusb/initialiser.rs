use crate::common::executor::InitialisableGoXLR;
use crate::platform::rusb::device::{GoXLRDevice, ReadControl, WriteControl};
use anyhow::bail;
use log::{debug, info};
use rusb::Error::Pipe;
use std::time::Duration;
use tokio::time::sleep;

impl InitialisableGoXLR for GoXLRDevice {
    async fn initialise(&mut self) -> anyhow::Result<()> {
        // This command 'resets' the GoXLR to a clean state..
        let reset_control = WriteControl {
            request: 1,
            value: 0,
            index: 0,
            data: &[],
        };

        // Attempt to execute it..
        let result = self.write_vendor_control(reset_control);
        if result == Err(Pipe) {
            // The GoXLR is not initialised, we need to fix that..
            info!("Found uninitialised GoXLR, attempting initialisation..");
            self.handle.set_auto_detach_kernel_driver(true)?;

            if self.handle.claim_interface(0).is_err() {
                bail!("Unable to Claim Device");
            }

            debug!("Activating Vendor Interface...");
            self.read_control(ReadControl {
                request: 0,
                value: 0,
                index: 0,
                length: 24,
            })?;

            // Now activate audio..
            debug!("Activating Audio...");
            self.write_class_control(WriteControl {
                request: 1,
                value: 0x0100,
                index: 0x2900,
                data: &[0x80, 0xbb, 0x00, 0x00],
            })?;

            self.handle.release_interface(0)?;

            // Reset the device, so ALSA can pick it up again..
            self.handle.reset()?;

            // We sleep for two seconds here, firstly so that Linux can internally re-grab the
            // device, and so that we don't interrupt any startup calibration.
            sleep(Duration::from_secs(2)).await;

            // Reattempt the reset..
            self.write_vendor_control(reset_control)?;
        }

        // Force command pipe activation in all cases.
        debug!("Handling initial request");
        let init = ReadControl {
            request: 3,
            value: 0,
            index: 0,
            length: 1040,
        };
        self.read_control(init)?;
        Ok(())
    }
}
