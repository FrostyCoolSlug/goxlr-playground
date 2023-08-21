use std::io::Cursor;

use anyhow::Result;
use async_trait::async_trait;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use enum_map::EnumMap;
use enumset::EnumSet;
use goxlr_shared::buttons::Buttons;
use strum::IntoEnumIterator;

use goxlr_shared::channels::{InputChannels, RoutingOutput};
use goxlr_shared::colours::{ColourScheme, FaderDisplayMode};
use goxlr_shared::faders::Fader;
use goxlr_shared::interaction::{ButtonStates, InteractiveButtons};
use goxlr_shared::routing::RouteValue;
use goxlr_shared::states::ButtonDisplayStates;
use goxlr_shared::version::{FirmwareVersions, VersionNumber};

use crate::common::executor::ExecutableGoXLR;
use crate::goxlr::commands::{Command, HardwareInfoCommand};
use crate::types::buttons::{CurrentButtonStates, PhysicalButton};
use crate::types::channels::AssignableChannel;
use crate::types::colours::ColourStruct;
use crate::types::faders::DeviceFader;
use crate::types::routing::RoutingChannel::{Left, Right};
use crate::types::routing::{RoutingInputChannel, RoutingOutputDevice};
use crate::types::states::ButtonDisplay;

type RoutingValues = EnumMap<RoutingOutput, RouteValue>;

#[async_trait]
/// This extension applies to anything that's implemented ExecutableGoXLR, and contains
/// all the specific command executors.
pub(crate) trait GoXLRCommands: ExecutableGoXLR {
    async fn get_serial_data(&mut self) -> Result<(String, String)> {
        let result = self
            .request_data(
                Command::GetHardwareInfo(HardwareInfoCommand::SerialNumber),
                &[],
            )
            .await?;

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
        let result = self
            .request_data(
                Command::GetHardwareInfo(HardwareInfoCommand::FirmwareVersion),
                &[],
            )
            .await?;
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

        for button in EnumSet::<PhysicalButton>::all() {
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

    async fn assign_fader(&mut self, fader: DeviceFader, source: AssignableChannel) -> Result<()> {
        // This could be simpler by doing: data = [source as u8, 0x00, 0x00, 0x00]
        // But I'm trying to make it clearer how data is handled.

        let mut data = [0; 4];
        LittleEndian::write_u32(&mut data, source as u32);

        self.request_data(Command::SetFader(fader), &data).await?;
        Ok(())
    }

    async fn apply_routing(&mut self, input: InputChannels, values: RoutingValues) -> Result<()> {
        // We need to take the values map, iterate it, and create the routing structure..
        let mut l_data = [0; 22];
        let mut r_data = [0; 22];

        for output in RoutingOutput::iter() {
            // We need to check the mapping of this value, and apply it..
            let left = RoutingOutputDevice::from(output, Left).position();
            let right = RoutingOutputDevice::from(output, Right).position();

            let value = values[output];

            l_data[left] = match value {
                RouteValue::On => 0x20,
                RouteValue::Off => 0x00,
                RouteValue::Value(value) => value,
            };

            r_data[right] = match value {
                RouteValue::On => 0x20,
                RouteValue::Off => 0x00,
                RouteValue::Value(value) => value,
            };
        }

        let left = RoutingInputChannel::from(input, Left);
        let right = RoutingInputChannel::from(input, Right);

        self.request_data(Command::SetRouting(left), &l_data)
            .await?;
        self.request_data(Command::SetRouting(right), &r_data)
            .await?;

        Ok(())
    }

    async fn apply_colour_scheme(&mut self, scheme: ColourScheme) -> Result<()> {
        let data = scheme.build_colour_map();
        self.request_data(Command::SetColourMap(), &data).await?;
        Ok(())
    }

    async fn set_fader_style(&mut self, fader: Fader, style: Vec<FaderDisplayMode>) -> Result<()> {
        // We can cast these straight from bools to u8s..
        let gradient = style.contains(&FaderDisplayMode::Gradient) as u8;
        let meter = style.contains(&FaderDisplayMode::Meter) as u8;

        let command = Command::SetFaderDisplayMode(fader.into());
        let data = [gradient, meter];

        // Send it!
        self.request_data(command, &data).await?;

        Ok(())
    }

    async fn set_button_states(&mut self, states: ButtonDisplayStates) -> Result<()> {
        // Create the base set with all buttons 'Dimmed'
        let mut state = [ButtonDisplay::DimmedColour1 as u8; 24];

        let buttons = states.get_list();
        for button in Buttons::iter() {
            let button_state: ButtonDisplay = buttons[button].into();
            let button_index: PhysicalButton = button.into();
            state[button_index as usize] = button_state as u8;
        }

        let command = Command::SetButtonStates();
        self.request_data(command, &state).await?;

        Ok(())
    }
}
