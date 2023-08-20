use std::io::Cursor;
use anyhow::Result;

use async_trait::async_trait;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use enumset::EnumSet;
use goxlr_shared::version::{FirmwareVersions, VersionNumber};

use crate::common::executor::ExecutableGoXLR;
use crate::goxlr::commands::{Command, HardwareInfoCommand};
use crate::types::buttons::{CurrentButtonStates, StatusButton};

#[async_trait]
/// This extension applies to anything that's implemented ExecutableGoXLR, and contains
/// all the specific command executors.
pub(crate) trait GoXLRCommands: ExecutableGoXLR {
    async fn get_serial_data(&mut self) -> Result<(String, String)> {
        let result = self.request_data(
            Command::GetHardwareInfo(HardwareInfoCommand::SerialNumber),
            &[],
        ).await?;

        let serial_slice = &result[..24];
        let serial_len = serial_slice
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(serial_slice.len());
        let serial_number = String::from_utf8_lossy(&serial_slice[..serial_len]).to_string();

        let date_slice = &result[24..];
        let date_len = date_slice
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(date_slice.len());
        let manufacture_date = String::from_utf8_lossy(&date_slice[..date_len]).to_string();

        Ok((serial_number, manufacture_date))
    }

    async fn get_firmware_version(&mut self) -> Result<FirmwareVersions> {
        let result = self.request_data(
            Command::GetHardwareInfo(HardwareInfoCommand::FirmwareVersion),
            &[],
        ).await?;
        let mut cursor = Cursor::new(result);
        let firmware_packed = cursor.read_u32::<LittleEndian>()?;
        let firmware_build = cursor.read_u32::<LittleEndian>()?;
        let firmware = VersionNumber(
            firmware_packed >> 12,
            (firmware_packed >> 8) & 0xF,
            firmware_packed & 0xFF,
            firmware_build,
        );

        let _unknown = cursor.read_u32::<LittleEndian>()?;
        let fpga_count = cursor.read_u32::<LittleEndian>()?;

        let dice_build = cursor.read_u32::<LittleEndian>()?;
        let dice_packed = cursor.read_u32::<LittleEndian>()?;
        let dice = VersionNumber(
            (dice_packed >> 20) & 0xF,
            (dice_packed >> 12) & 0xFF,
            dice_packed & 0xFFF,
            dice_build,
        );

        Ok(FirmwareVersions {
            firmware,
            fpga_count,
            dice,
        })
    }

    async fn get_button_states(&mut self) -> Result<CurrentButtonStates> {
        let result = self.request_data(Command::GetButtonStates, &[]).await?;
        let mut pressed = EnumSet::empty();
        let mut mixers = [0; 4];
        let mut encoders = [0; 4];
        let button_states = LittleEndian::read_u32(&result[0..4]);

        mixers[0] = result[8];
        mixers[1] = result[9];
        mixers[2] = result[10];
        mixers[3] = result[11];

        // These can technically be negative, cast straight to i8
        encoders[0] = result[4] as i8; // Pitch
        encoders[1] = result[5] as i8; // Gender
        encoders[2] = result[6] as i8; // Reverb
        encoders[3] = result[7] as i8; // Echo

        for button in EnumSet::<StatusButton>::all() {
            if button_states & (1 << button as u8) != 0 {
                pressed.insert(button);
            }
        }

        Ok(CurrentButtonStates {
            pressed,
            volumes: mixers,
            encoders,
        })
    }
}
