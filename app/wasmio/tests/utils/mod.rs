use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

use tokio::sync::OnceCell;

mod port_picker;

static CACHE: OnceCell<String> = OnceCell::const_new();
/// Start a server if needed
pub async fn start_simple_server() -> anyhow::Result<String> {
    use wasmio::config::{Cfg, StorageConfig};
    use wasmio::launch_wasmio;

    use crate::utils::port_picker::pick_unused_port;

    let e2e = std::env::var("E2E_ADDR").unwrap_or_default();

    if CACHE.initialized() {
        Ok(CACHE.get().cloned().unwrap())
    } else {
        let result = if !e2e.is_empty() {
            Ok::<_, anyhow::Error>(e2e)
        } else {
            let addr = SocketAddr::new(
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                pick_unused_port().unwrap(),
            );
            #[cfg(target_os = "wasi")]
            let path = std::path::PathBuf::from_str("/").unwrap();
            #[cfg(not(target_os = "wasi"))]
            let path = tempfile::tempdir().unwrap().path().into();
            std::fs::create_dir_all(&path).expect("shouldn't fail");
            let cfg = Cfg {
                bind_addr: addr,
                storage: StorageConfig { path },
            };
            tokio::spawn(async move {
                std::env::set_var("RUST_LOG", "info");
                launch_wasmio(cfg).await
            });

            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok(addr.to_string())
        };

        let result = result.unwrap();
        CACHE.set(result.clone())?;
        Ok(result)
    }
}
