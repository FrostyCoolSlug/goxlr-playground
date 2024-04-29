use log::debug;

use goxlr_ipc::commands::channels::{ChannelCommands, SubMixCommands};
use goxlr_ipc::commands::GoXLRCommandResponse;

use crate::device::goxlr::components::channel::Channels;
use crate::device::goxlr::components::mute_handler::MuteHandler;
use crate::device::goxlr::components::submix::SubMix;
use crate::device::goxlr::device::GoXLR;
use crate::device::goxlr::ipc::handler::Response;

type Command = ChannelCommands;

pub trait IPCChannelHandler {
    async fn ipc_channel(&mut self, command: Command) -> Response;
}

impl IPCChannelHandler for GoXLR {
    async fn ipc_channel(&mut self, command: Command) -> Response {
        match command {
            Command::Volume(params) => {
                self.set_channel_volume(params.channel, params.volume)
                    .await?;
            }
            Command::Mute(params) => {
                debug!("Applying Mute State..");
                self.set_mute_state(params.channel, params.state).await?;
            }

            Command::SubMix(command) => {
                let channel = command.channel;
                match command.command {
                    SubMixCommands::Volume(volume) => {
                        self.set_sub_mix_volume(channel, volume).await?;
                    }
                    SubMixCommands::Linked(linked) => {
                        self.set_sub_mix_linked(channel, linked).await?;
                    }
                }
            }
        }
        Ok(GoXLRCommandResponse::Ok)
    }
}
