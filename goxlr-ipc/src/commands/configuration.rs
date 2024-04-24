use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigurationCommand {
    SubMixEnabled(bool),
    ButtonHoldTime(u16),
    ChangePageWithButtons(bool),
}
