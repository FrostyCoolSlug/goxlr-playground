use crate::device::goxlr::components::submix::SubMix;
use crate::device::goxlr::device::GoXLR;
use anyhow::{bail, Context, Result};
use goxlr_shared::device::GoXLRFeature;
use goxlr_shared::faders::FaderSources;
use goxlr_usb::events::commands::BasicResultCommand;
use goxlr_usb::events::commands::ChannelSource;
use log::debug;

pub(crate) trait Channels {
    async fn set_channel_volume(&mut self, source: FaderSources, volume: u8) -> Result<()>;
    async fn apply_channel_volume(&mut self, source: FaderSources) -> Result<()>;
    async fn sync_channel_volume(&mut self, source: FaderSources) -> Result<()>;
}

impl Channels for GoXLR {
    async fn set_channel_volume(&mut self, source: FaderSources, volume: u8) -> Result<()> {
        self.profile.channels[source].volume.mix_a = volume;
        self.apply_channel_volume(source).await
    }

    async fn apply_channel_volume(&mut self, source: FaderSources) -> Result<()> {
        let target = ChannelSource::FromFaderSource(source);
        let volume = self.profile.channels[source].volume.mix_a;

        debug!("Setting Volume for {:?} from to {:?}", source, volume);

        let command = BasicResultCommand::SetVolume(target, volume);
        self.send_no_result(command).await?;

        self.sync_sub_mix_volume(source).await
    }

    async fn sync_channel_volume(&mut self, source: FaderSources) -> Result<()> {
        // Make sure submixes are even available on this device...
        let device = self.device.as_ref().context("Device not Set!")?;

        // Grab the linked ratio (If we're None, ignore)
        if let Some(linked) = self.profile.channels[source].volume.linked {
            // Because we're SubMix to Volume, we need to divide by the linked value
            let mix_volume = self.profile.channels[source].volume.mix_b;
            let linked_volume = (mix_volume as f64 / linked) as u8;

            self.profile.channels[source].volume.mix_a = linked_volume;

            // Bail at this point if Sub Mixes aren't supported.
            if !device.features.contains(&GoXLRFeature::Submix) {
                return Ok(());
            }

            let target = ChannelSource::FromFaderSource(source);
            let command = BasicResultCommand::SetVolume(target, linked_volume);
            return self.send_no_result(command).await;
        }

        // Volumes aren't linked, do nothing :)
        Ok(())
    }
}

pub(crate) trait ChannelsCrate {}
