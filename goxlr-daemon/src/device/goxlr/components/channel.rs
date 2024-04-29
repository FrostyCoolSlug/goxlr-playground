use anyhow::{Context, Result};
use goxlr_shared::channels::CanFrom;
use log::debug;

use crate::device::goxlr::components::submix::SubMix;
use goxlr_shared::channels::fader::FaderChannels;
use goxlr_shared::channels::sub_mix::SubMixChannels;
use goxlr_shared::channels::volume::VolumeChannels;
use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::device::GoXLR;

type Source = FaderChannels;

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
        self.profile.channels.volumes[source] = volume;
        self.apply_channel_volume(source).await
    }

    async fn apply_channel_volume(&mut self, source: VolumeChannels) -> Result<()> {
        let volume = self.profile.channels.volumes[source];

        debug!("Setting Volume for {:?} from to {:?}", source, volume);
        let command = BasicResultCommand::SetVolume(source, volume);
        self.send_no_result(command).await?;

        // TODO: Only apply this on SubMix supported Channels!

        if SubMixChannels::can_from(source) {
            self.sync_sub_mix_volume(source.into()).await?;
        }

        Ok(())
    }

    async fn sync_mix_volume(&mut self, source: VolumeChannels) -> Result<()> {
        // Make sure submixes are even available on this device...
        let device = self.device.as_ref().context("Device not Set!")?;

        // Grab the linked ratio (If we're None, ignore)
        // if let Some(linked) = self.profile.channels.sub_mix[source.into()].linked {
        //     // Because we're SubMix to Volume, we need to divide by the linked value
        //     let mix_volume = self.profile.channels.sub_mix[source.into()].volume;
        //     let linked_volume = (mix_volume as f64 / linked) as u8;
        //
        //     self.profile.channels.volumes[source.into()] = linked_volume;
        //
        //     // Bail at this point if Sub Mixes aren't supported.
        //     if !device.features.contains(&GoXLRFeature::Submix) {
        //         return Ok(());
        //     }
        //
        //     let command = BasicResultCommand::SetVolume(source.into(), linked_volume);
        //     return self.send_no_result(command).await;
        // }

        // Volumes aren't linked, do nothing :)
        Ok(())
    }
}

pub(crate) trait ChannelsCrate {}
