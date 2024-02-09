use std::{collections::HashMap, path::PathBuf};

use axum::async_trait;
use chrono::{DateTime, Utc};
use futures::future::join;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::warn;

/// Implement this trait which define the backend storage used to store data
///
/// The storage is very simple for now
///
/// It supports creating a Database
///
/// - Storing elements in the database
/// - Storing metadata in the database, the idea for metadata is: it should be fast to manipulate and it'll allow us to create index based on those metadata.
#[async_trait]
pub trait BackendStorage: Send + Sync {
    /// To create a new database
    async fn new_database(&self, name: &str) -> anyhow::Result<DatabaseInfo>;

    /// To get database metadata, if None, database doesn't exist
    async fn database_metadata(&self, name: &str) -> anyhow::Result<Option<DatabaseInfo>>;

    /// List elements from the database,
    async fn list_element_in_database<R: AsyncRead + Unpin>(
        &self,
        db: &str,
        start_after: Option<&str>,
        name_elt: &str,
    ) -> anyhow::Result<ElementInfo>;

    /// Get element from the database,
    async fn get_element_in_database<T: AsyncWrite + Send + Unpin, S: AsRef<str>>(
        &self,
        db: &str,
        key: &str,
        writer: &mut T,
    ) -> anyhow::Result<Option<ElementInfo>>;

    /// Put an element inside database
    async fn insert_element_in_database<R: AsyncRead + Unpin + Send>(
        &self,
        db: &str,
        name_elt: &str,
        content: &mut R,
    ) -> anyhow::Result<ElementInfo>;

    /// Put an element inside database
    async fn delete_element_in_database<R: AsyncRead + Unpin + Send>(
        &self,
        db: &str,
        name_elt: &str,
    ) -> anyhow::Result<ElementInfo>;
}

/// List of database info available
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DatabaseInfo {
    name: String,
    number_element: u64,
    created_at: DateTime<Utc>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ElementInfo {
    name: String,
    size: usize,
    created_at: DateTime<Utc>,
    last_modified: DateTime<Utc>,
    checksum: String,
    version: usize,
    /// User metadata
    metadata: HashMap<String, String>,
}

// -------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct FSStorage {
    base_path: PathBuf,
}

impl FSStorage {
    pub fn new(base: PathBuf) -> Self {
        let _ = std::fs::create_dir(base.join("data"));

        Self { base_path: base }
    }
}

#[async_trait]
impl BackendStorage for FSStorage {
    async fn new_database(&self, name: &str) -> anyhow::Result<DatabaseInfo> {
        let db_info = DatabaseInfo {
            name: name.to_string(),
            number_element: 0,
            created_at: Utc::now(),
        };
        let write_metadata = tokio::fs::write(
            self.base_path.join(format!("{name}.meta")),
            serde_json::to_string(&db_info)?,
        );
        let write_dir = tokio::fs::create_dir(self.base_path.join(name));

        let (a, b) = join(write_metadata, write_dir).await;
        a?;
        b?;

        Ok(db_info)
    }

    async fn database_metadata(&self, name: &str) -> anyhow::Result<Option<DatabaseInfo>> {
        let ressource_path = self.base_path.join(format!("{name}.meta"));

        if let Err(err) = tokio::fs::metadata(&ressource_path).await {
            warn!("{err:?}");
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(ressource_path).await?;

        let data_info = serde_json::from_str(&content)?;
        Ok(Some(data_info))
    }

    async fn insert_element_in_database<R: AsyncRead + Unpin + Send>(
        &self,
        db: &str,
        name_elt: &str,
        content: &mut R,
    ) -> anyhow::Result<ElementInfo> {
        let ressource_path = self.base_path.join(format!("{db}/{name_elt}.0.part.0"));

        let mut file = tokio::fs::File::open(ressource_path).await?;
        tokio::io::copy(content, &mut file).await?;

        Ok(Default::default())
    }

    async fn get_element_in_database<T: AsyncWrite + Send + Unpin, S: AsRef<str>>(
        &self,
        db: &str,
        key: &str,
        writer: &mut T,
    ) -> anyhow::Result<Option<ElementInfo>> {
        unimplemented!()
    }

    async fn list_element_in_database<R: AsyncRead + Unpin>(
        &self,
        db: &str,
        start_after: Option<&str>,
        name_elt: &str,
    ) -> anyhow::Result<ElementInfo> {
        unimplemented!()
    }

    async fn delete_element_in_database<R: AsyncRead + Unpin>(
        &self,
        db: &str,
        name_elt: &str,
    ) -> anyhow::Result<ElementInfo> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn simple_db_with_fs() {
        let temp = tempdir().unwrap();
        let fs = FSStorage::new(temp.path().to_path_buf());

        let result = fs.new_database("test").await;
        assert!(result.is_ok());

        let check_metadata_info = fs.database_metadata("test").await;
        assert!(check_metadata_info.is_ok());
        let check_metadata_info = check_metadata_info.unwrap();
        assert!(check_metadata_info.is_some());
    }
}
