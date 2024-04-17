use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use log::debug;
use tokio::select;
use tokio::signal::windows::{ctrl_break, ctrl_c, ctrl_close, ctrl_logoff, ctrl_shutdown};

use crate::stop::Stop;

pub async fn spawn_platform_runtime(mut stop: Stop) -> Result<()> {
    // Grab an async shutdown event..
    let mut ctrl_break = ctrl_break()?;
    let mut ctrl_close = ctrl_close()?;
    let mut ctrl_shutdown = ctrl_shutdown()?;
    let mut ctrl_logoff = ctrl_logoff()?;
    let mut ctrl_c = ctrl_c()?;

    loop {
        select! {
            Some(_) = ctrl_c.recv() => {
                debug!("CTRL_C");
                stop.trigger();
            }
            Some(_) = ctrl_break.recv() => {
                debug!("CTRL_BREAK");
                stop.trigger();
            },
            Some(_) = ctrl_close.recv() => {
                debug!("CTRL_CLOSE");
                stop.trigger();
            }
            Some(_) = ctrl_shutdown.recv() => {
                debug!("CTRL_SHUTDOWN");
                stop.trigger();
            }
            Some(_) = ctrl_logoff.recv() => {
                debug!("CTRL_LOGOFF");
                stop.trigger();
            }
            //Some(_) = ctrl_
            () = stop.recv() => {
                debug!("Stopping..");
                break;
            }
        };
    }

    Ok(())
}
