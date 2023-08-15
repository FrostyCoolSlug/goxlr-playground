use crate::USBLocation;
use log::debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::{join, select, task, time};

pub(crate) struct GoXLRDevice {
    config: GoXLRConfiguration,
    stop: Arc<AtomicBool>,
    task: Option<JoinHandle<()>>,
}

impl GoXLRDevice {
    pub async fn new(config: GoXLRConfiguration) -> Self {
        Self {
            config,
            stop: Arc::new(AtomicBool::new(false)),
            task: None,
        }
    }

    pub async fn initialise(&mut self) {
        // We need to load our GoXLR stuff, init the device, then return..
        debug!("[DEVICE]{} Initialising", self.config.device);

        let device = self.config.device;
        let stop = self.stop.clone();
        let events = self.config.events.clone();
        // Once we're done with that, spawn an event handler..
        self.task = Some(task::spawn(async move {
            debug!("[DEVICE]{} Spawning Event Loop..", device);
            let mut ticker = time::interval(Duration::from_millis(2000));
            loop {
                select! {
                    _ = ticker.tick() => {
                        if stop.load(Ordering::Relaxed) {
                            debug!("[DEVICE]{} Stopping Event Loop..", device);
                            break;
                        }
                        let _ = events.send(true).await;
                        debug!("[DEVICE]{} Event Loop Tick..", device);
                    }
                }
            }
            debug!("[DEVICE]{} Event Loop Stopped", device);
        }));
    }

    pub async fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);

        // Rejoin on the task, and hold the stop request until we're finished..
        if self.task.is_some() {
            let _ = self.task.take().unwrap().await;
        }
    }
}

pub struct GoXLRConfiguration {
    pub(crate) device: USBLocation,
    pub(crate) messenger: mpsc::Sender<bool>,
    pub(crate) events: mpsc::Sender<bool>,
}
