use std::io::{Cursor, Write};

use anyhow::Result;
use async_trait::async_trait;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use enum_map::EnumMap;
use enumset::EnumSet;
use strum::IntoEnumIterator;

use goxlr_shared::buttons::Buttons;
use goxlr_shared::channels::{InputChannels, RoutingOutput};
use goxlr_shared::colours::{ColourScheme, FaderDisplayMode};
use goxlr_shared::faders::Fader;
use goxlr_shared::routing::RouteValue;
use goxlr_shared::states::ButtonDisplayStates;
use goxlr_shared::version::{FirmwareVersions, VersionNumber};

use crate::common::executor::ExecutableGoXLR;
use crate::goxlr::commands::{Command, HardwareInfoCommand};
use crate::types::buttons::{CurrentButtonStates, DeviceButton};
use crate::types::channels::{AssignableChannel, ChannelState};
use crate::types::colours::ColourStruct;
use crate::types::faders::DeviceFader;
use crate::types::mic_keys::MicrophoneParamKeys;
use crate::types::microphone::MicrophoneType;
use crate::types::routing::RoutingChannel::{Left, Right};
use crate::types::routing::{RoutingInputChannel, RoutingOutputDevice};
use crate::types::states::ButtonDisplay;

type RoutingValues = EnumMap<RoutingOutput, RouteValue>;
type Channel = AssignableChannel;

type EffectKeys = goxlr_shared::microphone::MicrophoneEffectKey;
type ParamKeys = goxlr_shared::microphone::MicrophoneParamKeys;
type MicType = goxlr_shared::microphone::MicrophoneType;

/// This extension applies to anything that's implemented ExecutableGoXLR, and contains
/// all the specific command executors.
#[async_trait]
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

        for button in EnumSet::<DeviceButton>::all() {
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

    async fn assign_fader(&mut self, fader: DeviceFader, source: Channel) -> Result<()> {
        // This could be simpler by doing: data = [source as u8, 0x00, 0x00, 0x00]
        // But I'm trying to make it clearer how data is handled.

        let mut data = [0; 4];
        LittleEndian::write_u32(&mut data, source as u32);

        self.request_data(Command::SetFader(fader), &data).await?;
        Ok(())
    }

    async fn set_volume(&mut self, target: Channel, volume: u8) -> Result<()> {
        let command = Command::SetChannelVolume(target);
        self.request_data(command, &[volume]).await?;
        Ok(())
    }

    async fn set_mute_state(&mut self, target: Channel, state: ChannelState) -> Result<()> {
        let command = Command::SetChannelState(target);
        self.request_data(command, &[state as u8]).await?;
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
        let mut state = [ButtonDisplay::default() as u8; 24];

        let buttons = states.get_list();
        for button in Buttons::iter() {
            let button_state: ButtonDisplay = buttons[button].into();
            let button_index: DeviceButton = button.into();
            state[button_index as usize] = button_state as u8;
        }

        let command = Command::SetButtonStates();
        self.request_data(command, &state).await?;

        Ok(())
    }

    async fn set_scribble(&mut self, fader: Fader, data: [u8; 1024]) -> Result<()> {
        let command = Command::SetScribble(fader.into());
        self.request_data(command, &data).await?;

        Ok(())
    }

    async fn get_microphone_level(&mut self) -> Result<f64> {
        let result = self.request_data(Command::GetMicrophoneLevel, &[]).await?;
        let value = LittleEndian::read_u16(&result);

        // Convert the Value to Decibels
        let decibels = (f64::log(value.into(), 10.) * 20.) - 72.2;
        Ok(decibels.clamp(-72.2, 0.))
    }

    /// Microphone Stuff..
    async fn set_microphone_gain(&mut self, mic_type: MicType, gain: u8) -> Result<()> {
        let mic_type = MicrophoneType::from(mic_type);

        let mut gain_value = [0; 4];
        LittleEndian::write_u32(&mut gain_value, gain as u32);

        self.set_mic_param(&[
            (
                ParamKeys::MicType,
                match mic_type.has_phantom() {
                    true => [0x01, 0x00, 0x00, 0x00],
                    false => [0x00, 0x00, 0x00, 0x00],
                },
            ),
            (mic_type.get_gain_param(), gain_value),
        ])
        .await?;

        Ok(())
    }

    async fn set_mic_param(&mut self, params: &[(ParamKeys, [u8; 4])]) -> Result<()> {
        let mut data = Vec::with_capacity(params.len() * 8);
        let mut cursor = Cursor::new(&mut data);
        for (key, value) in params {
            let key = MicrophoneParamKeys::from(*key);
            cursor.write_u32::<LittleEndian>(key as u32)?;
            cursor.write_all(value)?;
        }
        self.request_data(Command::SetMicrophoneParameters, &data)
            .await?;

        Ok(())
    }
}
