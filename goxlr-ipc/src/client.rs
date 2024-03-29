use crate::commands::{DaemonRequest, DaemonStatus, GoXLRCommand};
use anyhow::Result;

pub trait Client {
    async fn send(&mut self, request: DaemonRequest) -> Result<()>;
    async fn poll_status(&mut self) -> Result<()>;
    async fn command(&mut self, serial: &str, command: GoXLRCommand) -> Result<()>;
    fn status(&self) -> &DaemonStatus;
}
