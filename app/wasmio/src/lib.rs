#![allow(dead_code)]
#![cfg_attr(all(target_arch = "wasm32", target_os = "wasi"), feature(wasi_ext))]
#![cfg_attr(
    all(target_arch = "wasm32", target_os = "wasi"),
    feature(async_fn_in_trait)
)]

mod application;
use application::Application;

mod domain;

mod infrastructure;
pub use infrastructure::config::{self, Cfg};
use infrastructure::instrumentation::Instruments;
use infrastructure::storage::FSStorage;
use tracing::info;

pub async fn launch_wasmio(cfg: Cfg) -> anyhow::Result<()> {
    // Instrumentation
    let _ = Instruments::new();
    info!("Starting the process");

    // Initiate the storage, we only support FS for now
    let storage = FSStorage::new(cfg.storage.path);

    // Server
    let app = Application::new(storage).serve(cfg.bind_addr);
    app.await??;

    info!("Ending the process");
    Ok(())
}
