use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigurationCommand {
    ButtonHoldTime(u16),
    ChangePageWithButtons(bool),
}
