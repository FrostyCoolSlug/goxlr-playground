use anyhow::{Context, Result};
use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::channels::volume::VolumeChannels;
use log::debug;

use goxlr_shared::device::GoXLRFeature;
use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::components::submix::SubMix;
use crate::device::goxlr::device::GoXLR;

type Source = FaderChannels;

pub(crate) trait Channels {
    /// Sets and applies a Channel Volume in the Profile
    async fn set_channel_volume(&mut self, source: Source, volume: u8) -> Result<()>;

    /// Applies a volume as set in the Profile
    async fn apply_channel_volume(&mut self, source: Source) -> Result<()>;

    /// Syncs a Channel Volume with it's SubMix volume
    async fn sync_mix_volume(&mut self, source: Source) -> Result<()>;
}

impl Channels for GoXLR {
    async fn set_channel_volume(&mut self, source: Source, volume: u8) -> Result<()> {
        self.profile.channels[source].volume.mix_a = volume;
        self.apply_channel_volume(source).await
    }

    async fn apply_channel_volume(&mut self, source: Source) -> Result<()> {
        let volume = self.profile.channels[source].volume.mix_a;

        debug!("Setting Volume for {:?} from to {:?}", source, volume);
        let target = VolumeChannels::from(source);
        let command = BasicResultCommand::SetVolume(target, volume);
        self.send_no_result(command).await?;

        if source.has_sub_mix() {
            self.sync_sub_mix_volume(source).await?;
        }
        Ok(())
    }

    async fn sync_mix_volume(&mut self, source: Source) -> Result<()> {
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

            let target = VolumeChannels::from(source);
            let command = BasicResultCommand::SetVolume(target, linked_volume);
            return self.send_no_result(command).await;
        }

        // Volumes aren't linked, do nothing :)
        Ok(())
    }
}

pub(crate) trait ChannelsCrate {}
