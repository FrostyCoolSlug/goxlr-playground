use anyhow::Result;
use async_trait::async_trait;

use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::device::GoXLR;

#[async_trait]
pub(crate) trait ButtonHandlers {
    async fn apply_button_states(&self) -> Result<()>;
}

#[async_trait]
impl ButtonHandlers for GoXLR {
    async fn apply_button_states(&self) -> Result<()> {
        let command = BasicResultCommand::SetButtonStates(self.button_states);
        self.send_no_result(command).await
    }
}
