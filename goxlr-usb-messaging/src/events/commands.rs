use anyhow::Result;

use goxlr_shared::colours::ColourScheme;
use tokio::sync::oneshot;

/// This is a helper enum for commands that will simply return a Result<()> with no additional
/// data, it helps simplify wrapping these type of commands together.
#[derive(Copy, Clone)]
pub enum BasicResultCommand {
    SetColour(ColourScheme),
}

pub enum CommandSender {
    BasicResultCommand(BasicResultCommand, oneshot::Sender<Result<()>>),
}
