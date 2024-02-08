use std::net::SocketAddr;

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
    pub fn from_env() -> anyhow::Result<Cfg> {
        let file_location = dotenv::var("CONFIG_FILE_LOCATION")
            .with_context(|| "`CONFIG_FILE_LOCATION` must be set.")?;

        let settings = Config::builder()
            .add_source(config::File::with_name(&file_location))
            .add_source(
                config::Environment::with_prefix("ROSTER")
                    .try_parsing(false)
                    .separator("_"),
            )
            .build()?;

        let config = settings.try_deserialize::<Cfg>()?;

        Ok(config)
    }
}
