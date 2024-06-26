use std::collections::BTreeMap;

use json_patch::Patch;
use serde::{Deserialize, Serialize};

use crate::commands::channels::ChannelCommands;
use crate::commands::configuration::ConfigurationCommand;
use crate::commands::mic::MicrophoneCommand;
use crate::commands::pages::PageCommand;
use crate::status::DeviceStatus;

pub mod channels;
pub mod configuration;
pub mod mic;
pub mod pages;

/// This is the base IPC request structure, it's async driven so each request will require a
/// response 'oneshot' channel for receiving a reply, this allows us to better manage a request /
/// response queued
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonRequest {
    /// Simple ping, will get an Ok / Error response
    Ping,

    /// This fetches the full status for all devices
    GetStatus,

    Daemon(DaemonCommand),
    DeviceCommand(DeviceCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketRequest {
    pub id: u64,
    pub data: DaemonRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonResponse {
    Ok,
    Err(String),
    Patch(Patch),
    Status(DaemonStatus),
    DeviceCommand(GoXLRCommandResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsocketResponse {
    pub id: u64,
    pub data: DaemonResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonCommand {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCommand {
    pub serial: String,
    pub command: GoXLRCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoXLRCommand {
    Configuration(ConfigurationCommand),
    Microphone(MicrophoneCommand),
    Channels(ChannelCommands),
    Pages(PageCommand),
}

/// The GoXLR Command Response will contain command specific responses, generally not much more
/// than 'Ok' in most cases, but if needed, we can provide more details messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoXLRCommandResponse {
    Ok,
    MicLevel(f64),
    Error(String),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DaemonStatus {
    pub devices: BTreeMap<String, DeviceStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpSettings {
    pub enabled: bool,
    pub bind_address: String,
    pub cors_enabled: bool,
    pub port: u16,
}
