use anyhow::Result;
use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::device::GoXLR;

pub(crate) trait ButtonHandlers {
    async fn apply_button_states(&self) -> Result<()>;
}

impl ButtonHandlers for GoXLR {
    async fn apply_button_states(&self) -> Result<()> {
        let command = BasicResultCommand::SetButtonStates(self.button_states);
        self.send_no_result(command).await
    }
}
