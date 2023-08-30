use crate::device::goxlr::device::GoXLR;
use anyhow::Result;
use async_trait::async_trait;
use goxlr_shared::buttons::{ButtonActiveState, Buttons};
use goxlr_usb_messaging::events::commands::BasicResultCommand;

#[async_trait]
pub(crate) trait ButtonHandlers {
    async fn apply_button_states(&self) -> Result<()>;
    fn set_button_state(&mut self, button: Buttons, state: ButtonActiveState) -> Result<()>;
}

#[async_trait]
impl ButtonHandlers for GoXLR {
    async fn apply_button_states(&self) -> Result<()> {
        let command = BasicResultCommand::SetButtonStates(self.button_states);
        self.send_no_result(command).await
    }

    fn set_button_state(&mut self, button: Buttons, state: ButtonActiveState) -> Result<()> {
        // Where we get the actual behaviour from will depend on the profile, so we can dig
        // it up here.
        match button {
            Buttons::Swear => {}
            _ => {
                // We just assume
            }
        }
        Ok(())
    }
}
