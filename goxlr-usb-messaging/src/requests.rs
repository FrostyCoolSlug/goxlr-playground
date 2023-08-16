/*
   This enum is simply a list of requests that can be sent to the GoXLR, and their response types.
*/

use tokio::sync::oneshot;

#[derive(Debug)]
pub enum GoXLRMessage {
    /// Returns the button State of the GoXLR
    GetStatus(oneshot::Sender<GoXLRStatus>),
}

#[derive(Debug)]
pub struct GoXLRStatus {}
