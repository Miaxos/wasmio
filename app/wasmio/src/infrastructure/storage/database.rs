use chrono::{DateTime, Utc};

/// List of database info available
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct DatabaseInfo {
    pub name: String,
    pub number_element: u64,
    pub created_at: DateTime<Utc>,
}

impl DatabaseInfo {
    pub fn name(&self) -> &str {
        &self.name
    }
}
