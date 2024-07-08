use std::cmp;

use anyhow::{Context, Result};
use goxlr_shared::channels::output::OutputChannels;
use goxlr_shared::channels::sub_mix::SubMixChannels;
use goxlr_shared::channels::volume::VolumeChannels;
use goxlr_shared::channels::CanFrom;
use log::{debug, warn};
use strum::IntoEnumIterator;

use crate::device::goxlr::components::channel::Channels;
use goxlr_shared::device::GoXLRFeature;
use goxlr_shared::submix::Mix;
use goxlr_usb::events::commands::BasicResultCommand;

use crate::device::goxlr::device::GoXLR;

/*
    Ok, for some background on GoXLR sub-mix behaviour...

    On a SubMix firmware version, there's technically no such thing as 'enabled' or 'disabled',
    this behaviour is handled in software.

    When the Submixes are 'Disabled', the app will force all audio out through Mix A, and only
    change the Mix A volume on updates, this simulates the non-submix behaviours. The actual
    GoXLR command to set the Mix A volume is the same 0x806 command which has always been present.

    When the Sub Mixes are 'Enabled', the App simply updates the Mix settings to match the config,
    and sets all Mix B volumes to their respective volumes. The Command to set the Mix B volume is
    still a 0x806, but the Channel ID is offset by 0x0F

    It should be noted, that the 'SetMixAssignment' command requires a Vec of Mix::A, and Mix::B,
    so we can shortcut a lot of things here by simply calling the load_sub_mix_assignments function
    whenever anything changes.

    All these things are now properly handled in the USB crate, here we just send data :)
*/

pub trait SubMix {
    async fn set_sub_mix_mix(&mut self, channel: OutputChannels, mix: Mix) -> Result<()>;
    async fn set_sub_mix_volume(&mut self, channel: SubMixChannels, volume: u8) -> Result<()>;
    async fn set_sub_mix_linked(&mut self, channel: SubMixChannels, linked: bool) -> Result<()>;

    async fn sync_sub_mix_volume(&mut self, channel: SubMixChannels) -> Result<()>;
    async fn load_sub_mix_assignments(&mut self) -> Result<()>;
}

impl SubMix for GoXLR {
    async fn set_sub_mix_mix(&mut self, channel: OutputChannels, mix: Mix) -> Result<()> {
        self.profile.outputs[channel].mix_assignment = mix;
        self.load_sub_mix_assignments().await
    }

    async fn set_sub_mix_volume(&mut self, channel: SubMixChannels, volume: u8) -> Result<()> {
        self.profile.channels.sub_mix[channel].volume = volume;

        let command = BasicResultCommand::SetSubMixVolume(channel, volume);
        self.send_no_result(command).await?;

        // Now sync the Mix::A volume
        if VolumeChannels::can_from(channel) {
            self.sync_mix_volume(channel.into()).await?;
        }

        Ok(())
    }

    async fn set_sub_mix_linked(&mut self, channel: SubMixChannels, linked: bool) -> Result<()> {
        if !linked {
            self.profile.channels.sub_mix[channel].linked = None;
            return Ok(());
        }

        // Ok, grab the mix volumes, but force them both to be > 0..
        let a_volume = cmp::max(self.profile.channels.volumes[channel.into()], 1);
        let b_volume = cmp::max(self.profile.channels.sub_mix[channel].volume, 1);
        let ratio = b_volume as f64 / a_volume as f64;

        // Disable the link between the channels..
        self.profile.channels.sub_mix[channel].linked = Some(ratio);
        Ok(())
    }

    async fn sync_sub_mix_volume(&mut self, channel: SubMixChannels) -> Result<()> {
        debug!("Syncing Submix");
        let device = self.device.as_ref().context("Device not Set!")?;

        // Grab the linked ratio (If we're None, ignore)
        if let Some(linked) = self.profile.channels.sub_mix[channel].linked {
            // We're syncing against the main volume, so multiply by ratio
            let mix_volume = self.profile.channels.volumes[channel.into()];
            let linked_volume = (mix_volume as f64 * linked) as u8;

            // Set the new volume in the profile..
            self.profile.channels.sub_mix[channel].volume = linked_volume;

            // If submixes aren't supported, simply bail.
            if !device.features.contains(&GoXLRFeature::SubMix) {
                return Ok(());
            }

            let command = BasicResultCommand::SetSubMixVolume(channel, linked_volume);
            return self.send_no_result(command).await;
        }

        Ok(())
    }

    async fn load_sub_mix_assignments(&mut self) -> Result<()> {
        let device = self.device.as_ref().context("Device Not Set!")?;
        if !device.features.contains(&GoXLRFeature::SubMix) {
            warn!("Sub Mixing Not Available, not loading...");
            return Ok(());
        }

        // Ok, sub mixing is enabled, assign things to the correct channel
        let mut mix_a = vec![];
        let mut mix_b = vec![];

        // Iterate the outputs, and push them into the correct mix
        for channel in OutputChannels::iter() {
            match self.profile.outputs[channel].mix_assignment {
                Mix::A => mix_a.push(channel),
                Mix::B => mix_b.push(channel),
            }
        }

        // Send the command across
        let command = BasicResultCommand::SetSubMixMix(mix_a, mix_b);
        self.send_no_result(command).await
    }
}
