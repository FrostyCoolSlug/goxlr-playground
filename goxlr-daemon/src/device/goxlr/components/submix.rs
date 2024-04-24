use crate::device::goxlr::components::channel::Channels;
use crate::device::goxlr::device::GoXLR;
use anyhow::{Context, Result};
use goxlr_shared::channels::OutputChannels;
use goxlr_shared::device::GoXLRFeature;
use goxlr_shared::faders::FaderSources;
use goxlr_shared::submix::Mix;
use goxlr_usb::events::commands::{BasicResultCommand, ChannelSource};
use log::warn;
use strum::IntoEnumIterator;

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
    async fn set_sub_mix_enabled(&mut self, enabled: bool) -> Result<()>;
    async fn set_sub_mix_mix(&mut self, channel: OutputChannels, mix: Mix) -> Result<()>;
    async fn set_sub_mix_volume(&mut self, channel: FaderSources, volume: u8) -> Result<()>;

    async fn sync_sub_mix_volume(&mut self, channel: FaderSources) -> Result<()>;
    async fn load_sub_mix_assignments(&mut self) -> Result<()>;
}

impl SubMix for GoXLR {
    async fn set_sub_mix_enabled(&mut self, enabled: bool) -> Result<()> {
        // Simply set the value in the profile
        self.profile.configuration.submix_enabled = enabled;

        // Then load the mixes
        self.load_sub_mix_assignments().await
    }

    async fn set_sub_mix_mix(&mut self, channel: OutputChannels, mix: Mix) -> Result<()> {
        self.profile.outputs[channel].mix_assignment = mix;
        self.load_sub_mix_assignments().await
    }

    async fn set_sub_mix_volume(&mut self, channel: FaderSources, volume: u8) -> Result<()> {
        self.profile.channels[channel].volume.mix_b = volume;

        let source = ChannelSource::FromFaderSource(channel);
        let command = BasicResultCommand::SetSubMixVolume(source, volume);
        self.send_no_result(command).await?;

        // Now sync the Mix::A volume
        self.sync_channel_volume(channel).await
    }

    async fn sync_sub_mix_volume(&mut self, channel: FaderSources) -> Result<()> {
        let device = self.device.as_ref().context("Device not Set!")?;
        if !device.features.contains(&GoXLRFeature::Submix) {
            // We return OK here, because there's nothing to do if Sub Mixes aren't available
            return Ok(());
        }

        // Grab the linked ratio (If we're None, ignore)
        if let Some(linked) = self.profile.channels[channel].volume.linked {
            // We're syncing against the main volume, so multiply by ratio
            let mix_volume = self.profile.channels[channel].volume.mix_a;
            let linked_volume = (mix_volume as f64 * linked) as u8;

            let target = ChannelSource::FromFaderSource(channel);
            let command = BasicResultCommand::SetSubMixVolume(target, linked_volume);
            return self.send_no_result(command).await;
        }

        Ok(())
    }

    async fn load_sub_mix_assignments(&mut self) -> Result<()> {
        let device = self.device.as_ref().context("Device Not Set!")?;
        if !device.features.contains(&GoXLRFeature::Submix) {
            warn!("Sub Mixing Not Available, not loading...");
            return Ok(());
        }

        // Firstly, if sub mixing is disabled, force everything to Mix A
        if !self.profile.configuration.submix_enabled {
            // Sub mixing isn't enabled, force everything to Mix A
            let mut a = vec![];
            for channel in OutputChannels::iter() {
                a.push(channel);
            }
            let command = BasicResultCommand::SetSubMixMix(a, vec![]);
            return self.send_no_result(command).await;
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
