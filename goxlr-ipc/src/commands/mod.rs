use goxlr_profile::{MicProfile, Profile};
use serde::{Deserialize, Serialize};

use goxlr_shared::faders::FaderSources;

use crate::commands::channels::ChannelCommand;
use crate::commands::pages::PageCommand;

pub mod channels;
pub mod pages;

/// This is the base IPC request structure, it's async driven so each request will require a
/// response 'oneshot' channel for receiving a reply, this allows us to better manage a request /  
/// response queued
#[derive(Debug, Serialize, Deserialize)]
pub enum DaemonRequest {
    /// Simple ping, will get an Ok / Error response
    Ping,

    /// This fetches the full status for all devices
    GetStatus,

    Daemon(DaemonCommand),
    DeviceCommand(DeviceCommand),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DaemonResponse {
    Ok,
    Error(String),
    Status(DaemonStatus),
    Command(GoXLRCommandResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCommand {
    pub serial: String,
    pub command: GoXLRCommand,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DaemonCommand {}

#[derive(Debug, Serialize, Deserialize)]
pub enum GoXLRCommand {
    Channels(Channels),
    Pages(PageCommand),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channels {
    pub channel: FaderSources,
    pub command: ChannelCommand,
}

/// The GoXLR Command Response will contain command specific responses, generally not much more
/// than 'Ok' in most cases, but if needed, we can provide more details messages.
#[derive(Debug, Serialize, Deserialize)]
pub enum GoXLRCommandResponse {
    Ok,
    Error(String),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DaemonStatus {
    pub devices: Vec<DeviceStatus>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub serial: String,
    pub config: Profiles,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Profiles {
    pub profile: Profile,
    pub mic_profile: MicProfile,
}
