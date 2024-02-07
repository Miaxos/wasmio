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
"###,
        version = VERSION,
        port = config.bind_addr.port(),
        addr = config.bind_addr.ip(),
    );

    // Instrumentation
    let _ = Instruments::new();

    info!("Go");

    Ok(())
}
