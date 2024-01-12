use goxlr_shared::faders::{Fader, FaderSources};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageCommand {
    AddPage,
    LoadPage(PageNumber),
    RemovePage(PageNumber),
    SetFader(SetFader),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageNumber {
    pub page_number: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFader {
    pub page_number: u8,
    pub fader: Fader,
    pub channel: FaderSources,
}
