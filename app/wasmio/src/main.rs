mod infrastructure;
use infrastructure::config::Cfg;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Cfg::from_env()?;

    dbg!(config);

    println!("hello mom!");
    Ok(())
}
