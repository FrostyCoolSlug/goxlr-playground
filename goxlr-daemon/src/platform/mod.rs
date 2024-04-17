use crate::Stop;
use anyhow::Result;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(windows)] {
        mod windows;

        pub async fn spawn_runtime(stop: Stop) -> Result<()> {
            windows::spawn_platform_runtime(stop).await
        }
    } else if #[cfg(target_os = "linux")] {
        mod linux;

        pub async fn spawn_runtime(stop: Stop) -> Result<()> {
            linux::spawn_platform_runtime(stop).await
        }
    } else if #[cfg(target_os = "macos")] {
        mod macos;

        pub async fn spawn_runtime(_stop: Stop) -> Result<()> {
            Ok(())
        }
    } else {
        use anyhow::bail;

        pub async fn spawn_runtime(_stop: Stop) -> Result<()> {
            Ok(())
        }
    }
}
