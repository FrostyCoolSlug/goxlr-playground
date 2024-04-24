use crate::types::channels::AssignableChannel;
use crate::types::encoders::DeviceEncoder;
use crate::types::faders::DeviceFader;
use crate::types::routing::RoutingInputChannel;
use crate::types::submix::SubMixChannelName;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Command {
    ResetCommandIndex,
    SystemInfo(SystemInfoCommand),
    SetChannelState(AssignableChannel),
    SetChannelVolume(AssignableChannel),
    SetEncoderValue(DeviceEncoder),
    SetEncoderMode(DeviceEncoder),
    SetFader(DeviceFader),
    SetRouting(RoutingInputChannel),
    SetButtonStates(),
    SetEffectParameters,
    SetMicrophoneParameters,
    GetMicrophoneLevel,
    SetColourMap(),
    SetFaderDisplayMode(DeviceFader),
    SetScribble(DeviceFader),
    GetButtonStates,
    GetHardwareInfo(HardwareInfoCommand),

    SetAnimationMode,

    SetSubChannelVolume(AssignableChannel),
    SetChannelMixes,
    SetMonitoredMix,

    // Probably shouldn't use these, but they're here for.. reasons.
    ExecuteFirmwareUpdate(FirmwareCommand),
    ExecuteFirmwareUpdateAction(FirmwareAction),
}

impl Command {
    pub fn command_id(&self) -> u32 {
        match self {
            Command::ResetCommandIndex => 0,
            Command::SystemInfo(sub) => sub.id(),
            Command::SetChannelState(channel) => (0x809 << 12) | *channel as u32,
            Command::SetChannelVolume(channel) => (0x806 << 12) | *channel as u32,
            Command::SetEncoderValue(encoder) => (0x80a << 12) | *encoder as u32,
            Command::SetEncoderMode(encoder) => (0x811 << 12) | *encoder as u32,
            Command::SetFader(fader) => (0x805 << 12) | *fader as u32,
            Command::SetRouting(channel) => (0x804 << 12) | channel.id() as u32,
            Command::SetColourMap() => 0x803 << 12,
            Command::SetButtonStates() => 0x808 << 12,
            Command::SetFaderDisplayMode(fader) => (0x814 << 12) | *fader as u32,
            Command::SetScribble(fader) => (0x802 << 12) | *fader as u32,
            Command::GetButtonStates => 0x800 << 12,
            Command::GetHardwareInfo(sub) => (0x80f << 12) | *sub as u32,
            Command::GetMicrophoneLevel => 0x80c << 12,
            Command::SetMicrophoneParameters => 0x80b << 12,
            Command::SetEffectParameters => 0x801 << 12,

            // Animation Related Commands
            Command::SetAnimationMode => 0x816 << 12,

            // I'm doing a +0x0F here, because there appears to be a bit reset going on..
            Command::SetSubChannelVolume(channel) => (0x806 << 12) | *channel as u32 + 0x0f,
            Command::SetChannelMixes => 0x817 << 12,
            Command::SetMonitoredMix => 0x818 << 12,

            // Again, don't use these :)
            Command::ExecuteFirmwareUpdate(sub) => 0x810 << 12 | *sub as u32,
            Command::ExecuteFirmwareUpdateAction(sub) => 0x004 << 12 | sub.id(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SystemInfoCommand {
    FirmwareVersion,
    SupportsDCPCategory,
}

impl SystemInfoCommand {
    pub fn id(&self) -> u32 {
        match self {
            SystemInfoCommand::FirmwareVersion => 2,
            SystemInfoCommand::SupportsDCPCategory => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HardwareInfoCommand {
    FirmwareVersion = 0,
    SerialNumber = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FirmwareCommand {
    // Start the update (Makes GoXLR go green, we should lock the util here.)
    START,

    // Verify the update on the GoXLR, use POLL for progress
    VERIFY,

    // Aborts the Firmware update, only call at the *END* of VERIFY
    ABORT,

    // Writes the firmware to active memory, use POLL for progress
    FINALISE,

    // Reboots the GoXLR upon completion of the firmware update
    REBOOT,

    // Used for polling status (VERIFY / FINALISE)
    POLL,
}

// DCP Commands for managing a firmware update (0x004)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FirmwareAction {
    // Formats and erases the update partition
    ERASE,

    // Poll for ERASE to hit 0xFF
    POLL,

    // Sends a data chunk to the update partition
    SEND,

    // Receive Checksums and Validate data
    VALIDATE,
}

impl FirmwareAction {
    pub fn id(&self) -> u32 {
        match self {
            FirmwareAction::ERASE => 2,
            FirmwareAction::POLL => 3,
            FirmwareAction::SEND => 4,
            FirmwareAction::VALIDATE => 6,
        }
    }
}
