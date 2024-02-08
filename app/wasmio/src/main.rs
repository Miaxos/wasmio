#![cfg_attr(all(target_arch = "wasm32", target_os = "wasi"), feature(wasi_ext))]

mod application;
use application::Application;

mod domain;

mod infrastructure;
use infrastructure::config::Cfg;
use infrastructure::constant::VERSION;
use infrastructure::instrumentation::Instruments;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Cfg::from_env()?;

    println!(
        r###"
 ▄     ▄ ▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄   ▄▄ ▄▄▄ ▄▄▄▄▄▄▄ 
█ █ ▄ █ █      █       █  █▄█  █   █       █
█ ██ ██ █  ▄   █  ▄▄▄▄▄█       █   █   ▄   █
█       █ █▄█  █ █▄▄▄▄▄█       █   █  █ █  █
█       █      █▄▄▄▄▄  █       █   █  █▄█  █
█   ▄   █  ▄   █▄▄▄▄▄█ █ ██▄██ █   █       █
█▄▄█ █▄▄█▄█ █▄▄█▄▄▄▄▄▄▄█▄█   █▄█▄▄▄█▄▄▄▄▄▄▄█

Version: {version}
port: {port}
addr: {addr}

by @miaxos https://github.com/miaxos
"###,
        version = VERSION,
        port = config.bind_addr.port(),
        addr = config.bind_addr.ip(),
    );

    // Instrumentation
    let _ = Instruments::new();
    info!("Starting the process");

    for i in std::env::vars() {
        info!("{i:?}");
    }

    // Server
    let app = Application::new().serve(config.bind_addr);
    app.await??;

    info!("Ending the process");
    Ok(())
}
