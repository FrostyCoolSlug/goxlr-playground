use log::debug;

struct Device {}

impl Device {
    fn new() -> Self {
        Self {}
    }

    /// Called to run and handle device related tasks, including the first initialisation, status
    /// polling, and event management.
    async fn run() {
        debug!("Spinning up Linux event handler..");
    }
}
