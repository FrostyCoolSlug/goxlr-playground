use log::debug;

use crate::device::goxlr::components::channel::Channels;
use crate::device::goxlr::components::mute_handler::MuteHandler;
use crate::device::goxlr::components::submix::SubMix;
use goxlr_ipc::commands::channels::ChannelCommand;
use goxlr_ipc::commands::GoXLRCommandResponse;
use goxlr_shared::faders::FaderSources;

use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::handler::Response;

type Source = FaderSources;
type Command = ChannelCommand;

pub trait IPCChannelHandler {
    async fn ipc_channel(&mut self, channel: Source, command: Command) -> Response;
}

impl IPCChannelHandler for GoXLR {
    async fn ipc_channel(&mut self, channel: Source, command: Command) -> Response {
        match command {
            Command::Volume(volume) => {
                self.set_channel_volume(channel, volume).await?;
                Ok(GoXLRCommandResponse::Ok)
            }
            Command::SubVolume(volume) => {
                self.set_sub_mix_volume(channel, volume).await?;
                Ok(GoXLRCommandResponse::Ok)
            }
            Command::Mute(state) => {
                debug!("Applying Mute State..");
                self.set_mute_state(channel, state).await?;

                Ok(GoXLRCommandResponse::Ok)
            }
            Command::SubMixLinked(linked) => {
                self.set_sub_mix_linked(channel, linked).await?;
                Ok(GoXLRCommandResponse::Ok)
            }
        }
    }
}
