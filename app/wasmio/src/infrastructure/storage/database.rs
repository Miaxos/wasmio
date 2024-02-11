use chrono::{DateTime, Utc};

/// List of database info available
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct DatabaseInfo {
    pub name: String,
    pub number_element: u64,
    pub created_at: DateTime<Utc>,
}

impl DatabaseInfo {
    pub fn new_database(name: String) -> Self {
        Self {
            name,
            number_element: 0,
            created_at: Utc::now(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
