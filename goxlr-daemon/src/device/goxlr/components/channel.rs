use crate::device::goxlr::device::GoXLR;
use anyhow::Result;
use async_trait::async_trait;
use goxlr_shared::faders::FaderSources;
use goxlr_usb::events::commands::BasicResultCommand;
use goxlr_usb::events::commands::ChannelSource;
use log::debug;

#[async_trait]
pub(crate) trait Channels {
    async fn set_channel_volume(&self, source: FaderSources, volume: u8) -> Result<()>;
}

#[async_trait]
impl Channels for GoXLR {
    async fn set_channel_volume(&self, source: FaderSources, volume: u8) -> Result<()> {
        let target = ChannelSource::FromFaderSource(source);

        debug!("Setting Volume for {:?} from to {:?}", source, volume);

        let command = BasicResultCommand::SetVolume(target, volume);
        self.send_no_result(command).await
    }
}
