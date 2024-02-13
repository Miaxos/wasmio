use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;
use config::Config;
use serde::{Deserialize, Serialize};

mod storage;
pub use storage::StorageConfig;

/// Configuration file for the application.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cfg {
    pub bind_addr: SocketAddr,

    pub storage: StorageConfig,
}

impl Cfg {
    /// Read the associated configuration env
    #[allow(dead_code)]
    pub fn from_env() -> anyhow::Result<Cfg> {
        let file_location = dotenv::var("CONFIG_FILE_LOCATION")
            .with_context(|| "`CONFIG_FILE_LOCATION` must be set.")?;

        let settings = Config::builder()
            .add_source(config::File::with_name(&file_location))
            .add_source(
                config::Environment::with_prefix("WASMIO")
                    .try_parsing(false)
                    .separator("_"),
            )
            .build()?;

        let config = settings.try_deserialize::<Cfg>()?;

        Ok(config)
    }

    #[allow(dead_code)]
    pub fn hack() -> anyhow::Result<Cfg> {
        Ok(Cfg {
            bind_addr: SocketAddr::from_str("0.0.0.0:80")?,
            storage: StorageConfig {
                path: PathBuf::new().join("public").join("data"),
            },
        })
    }
}
