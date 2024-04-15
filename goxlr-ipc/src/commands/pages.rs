use goxlr_shared::faders::{Fader, FaderSources};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageCommand {
    AddPage,
    LoadPage(u8),
    RemovePage(u8),
    SetFader(SetFader),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFader {
    pub page_number: u8,
    pub fader: Fader,
    pub channel: FaderSources,
}
