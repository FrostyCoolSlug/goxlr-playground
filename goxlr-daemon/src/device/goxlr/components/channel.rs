use anyhow::{bail, Context, Result};
use goxlr_shared::channels::CanFrom;
use log::debug;

use crate::device::goxlr::components::has_feature;
use crate::device::goxlr::components::submix::SubMix;
use goxlr_shared::channels::sub_mix::SubMixChannels;
use goxlr_shared::channels::volume::VolumeChannels;
use goxlr_shared::device::GoXLRFeature;
use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::device::GoXLR;

pub(crate) trait Channels {
    /// Sets and applies a Channel Volume in the Profile
    async fn set_channel_volume(&mut self, source: VolumeChannels, volume: u8) -> Result<()>;

    /// Applies a volume as set in the Profile
    async fn apply_channel_volume(&mut self, source: VolumeChannels) -> Result<()>;

    /// Syncs a Channel Volume with it's SubMix volume
    async fn sync_mix_volume(&mut self, source: VolumeChannels) -> Result<()>;
}

impl Channels for GoXLR {
    async fn set_channel_volume(&mut self, source: VolumeChannels, volume: u8) -> Result<()> {
        let sub_mix = has_feature(&self.device, GoXLRFeature::SubMix)?;
        if sub_mix && source == VolumeChannels::MicrophoneMonitor {
            bail!("Cannot Adjust Mic Monitor when Submixes are available");
        }

        self.profile.channels.volumes[source] = volume;
        self.apply_channel_volume(source).await
    }

    async fn apply_channel_volume(&mut self, source: VolumeChannels) -> Result<()> {
        let mic_monitor = source == VolumeChannels::MicrophoneMonitor;
        let volume = if has_feature(&self.device, GoXLRFeature::SubMix)? && mic_monitor {
            // If sub-mixes are available, regardless of the config the MicMonitor channel needs
            // to be set to 100%. Monitoring should be adjusted by the mix.
            255
        } else {
            self.profile.channels.volumes[source]
        };

        debug!("Setting Volume for {:?} from to {:?}", source, volume);
        let command = BasicResultCommand::SetVolume(source, volume);
        self.send_no_result(command).await?;

        if SubMixChannels::can_from(source) {
            self.sync_sub_mix_volume(source.into()).await?;
        }

        Ok(())
    }

    async fn sync_mix_volume(&mut self, source: VolumeChannels) -> Result<()> {
        // Grab the linked ratio (If we're None, ignore)
        if let Some(linked) = self.profile.channels.sub_mix[source.into()].linked {
            // Because we're SubMix to Volume, we need to divide by the linked value
            let mix_volume = self.profile.channels.sub_mix[source.into()].volume;
            let linked_volume = (mix_volume as f64 / linked) as u8;

            self.profile.channels.volumes[source] = linked_volume;

            // Bail at this point if Sub Mixes aren't supported.
            if !has_feature(&self.device, GoXLRFeature::SubMix)? {
                return Ok(());
            }

            let command = BasicResultCommand::SetVolume(source, linked_volume);
            return self.send_no_result(command).await;
        }

        // Volumes aren't linked, do nothing :)
        Ok(())
    }
}

pub(crate) trait ChannelsCrate {}
