use anyhow::Result;
use cfg_if::cfg_if;
use crate::Stop;

cfg_if! {
    if #[cfg(windows)] {
        mod windows;

        pub async fn spawn_runtime(stop: Stop) -> Result<()> {
            windows::spawn_platform_runtime(stop).await
        }
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        mod unix;

        pub async fn spawn_runtime(state: DaemonState, tx: mpsc::Sender<EventTriggers>) -> Result<()> {
            Ok(())
        }
    } else if #[cfg(target_os = "macos")] {
        mod macos;

        pub async fn spawn_runtime(state: DaemonState, tx: mpsc::Sender<EventTriggers>) -> Result<()> {
            Ok(())
        }
    } else {
        use anyhow::bail;

        pub async fn spawn_runtime(_state: DaemonState, _tx: mpsc::Sender<EventTriggers>) -> Result<()> {
            Ok(())
        }
    }
}