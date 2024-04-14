use crate::device::goxlr::device::GoXLR;
use anyhow::Result;
use goxlr_shared::microphone::MicrophoneType;
use goxlr_usb::events::commands::BasicResultCommand;

pub trait MicType {
    async fn set_mic_type(&mut self, mic_type: MicrophoneType) -> Result<()>;
    async fn set_mic_gain(&mut self, gain: u8) -> Result<()>;
}

impl MicType for GoXLR {
    async fn set_mic_type(&mut self, mic_type: MicrophoneType) -> Result<()> {
        // This is relatively straight forward, send a Gain command for the new Mic type..
        self.mic_profile.microphone.mic_type = mic_type;
        self.apply_mic_gain().await
    }

    async fn set_mic_gain(&mut self, gain: u8) -> Result<()> {
        let current_type = self.mic_profile.microphone.mic_type;
        self.mic_profile.microphone.mic_gains[current_type] = gain;
        self.apply_mic_gain().await
    }
}

pub(crate) trait MicTypeCrate {
    async fn apply_mic_gain(&mut self) -> Result<()>;
}

impl MicTypeCrate for GoXLR {
    async fn apply_mic_gain(&mut self) -> Result<()> {
        let mic_type = self.mic_profile.microphone.mic_type;
        let gain = self.mic_profile.microphone.mic_gains[mic_type];

        let command = BasicResultCommand::SetMicGain(mic_type, gain);
        self.send_no_result(command).await?;

        Ok(())
    }
}

trait MicTypeLocal {}

impl MicTypeLocal for GoXLR {}
