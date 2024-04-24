use log::debug;

use crate::device::goxlr::components::channel::Channels;
use crate::device::goxlr::components::mute_handler::MuteHandler;
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
            Command::SetVolume(volume) => {
                self.profile.channels[channel].volume.mix_a = volume;
                self.set_channel_volume(channel, volume).await?;

                Ok(GoXLRCommandResponse::Ok)
            }
            Command::SetMute(state) => {
                debug!("Applying Mute State..");
                self.set_mute_state(channel, state).await?;

                Ok(GoXLRCommandResponse::Ok)
            }
        }
    }
}
