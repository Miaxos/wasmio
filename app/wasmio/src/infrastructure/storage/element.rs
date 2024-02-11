use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ElementInfo {
    pub name: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    /// Only using sha256 for now
    pub checksum: String,
    pub metadatas: HashMap<String, String>,
}
