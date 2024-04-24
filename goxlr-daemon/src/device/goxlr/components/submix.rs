use anyhow::Result;
use goxlr_profile::FaderChannel;
use goxlr_shared::channels::OutputChannels;
use goxlr_shared::submix::Mix;

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

*/

pub trait SubMix {
    async fn set_sub_mix_enabled(&mut self, enabled: bool) -> Result<()>;
    async fn set_sub_mix_mix(&mut self, channel: OutputChannels, mix: Mix) -> Result<()>;
    async fn set_sub_mix_volume(&mut self, channel: FaderChannel, volume: u8) -> Result<()>;

    async fn sync_sub_mix_volume(&mut self, channel: FaderChannel) -> Result<()>;
    async fn load_sub_mix_assignments(&mut self) -> Result<()>;
}
