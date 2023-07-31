use crate::GoXLRDevice;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

#[async_trait]
trait GoXLRPnPHandler {
    async fn run(sender: Sender<DeviceEvents>);
}

pub enum DeviceEvents {
    Attached(GoXLRDevice),
    Removed(GoXLRDevice),
    Error(GoXLRDevice),
}
