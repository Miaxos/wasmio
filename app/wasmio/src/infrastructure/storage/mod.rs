use std::path::PathBuf;

use axum::async_trait;
use base64ct::{Base64, Encoding};
use chrono::{DateTime, Utc};
use futures::{future::join, Stream, StreamExt, TryStreamExt};
use sha2::{digest::generic_array::GenericArray, Digest, Sha256};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
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
    ) -> anyhow::Result<Box<dyn Stream<Item = anyhow::Result<String>>>>;

    /// Get element from the database,
    async fn get_element_in_database<T: AsyncWrite + Send + Unpin, S: AsRef<str>>(
        &self,
        db: &str,
        key: &str,
        writer: &mut T,
    ) -> anyhow::Result<u64>;

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
    size: u64,
    created_at: DateTime<Utc>,
    /// Only using sha256 for now
    checksum: String,
}

// -------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct FSStorage {
    base_path: PathBuf,
}

impl FSStorage {
    pub fn new(base: PathBuf) -> Self {
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
        let now = Utc::now();

        let ressource_path = self.base_path.join(db).join(format!("{name_elt}.0.part.0"));
        let mut file_content = tokio::fs::File::create(ressource_path).await?;

        // TODO: test based on `cat public/data/test-bucket/test.txt.0.part.0 | openssl sha256 -binary | base64`
        let mut hasher = Sha256::new();

        let stream = tokio_util::io::ReaderStream::new(content);
        let mut ar = tokio_util::io::StreamReader::new(stream.map_ok(|x| {
            hasher.update(&*x);
            x
        }));

        let size = tokio::io::copy(&mut ar, &mut file_content).await?;
        let hash = Base64::encode_string(&hasher.finalize());

        let metadata_path = self.base_path.join(db).join(format!("{name_elt}.0.meta"));
        let elt = ElementInfo {
            name: name_elt.to_string(),
            size,
            created_at: now,
            checksum: hash,
        };

        tokio::fs::write(metadata_path, serde_json::to_string(&elt)?).await?;

        // TODO: Increasing the count
        Ok(elt)
    }

    async fn get_element_in_database<T: AsyncWrite + Send + Unpin, S: AsRef<str>>(
        &self,
        db: &str,
        key: &str,
        mut writer: &mut T,
    ) -> anyhow::Result<u64> {
        let ressource_path = self.base_path.join(db).join(format!("{key}.0.part.0"));
        let mut file_content = tokio::fs::File::open(ressource_path).await?;

        let size = tokio::io::copy(&mut file_content, &mut writer).await?;

        Ok(size)
    }

    async fn list_element_in_database<R: AsyncRead + Unpin>(
        &self,
        db: &str,
        _start_after: Option<&str>,
    ) -> anyhow::Result<Box<dyn Stream<Item = anyhow::Result<String>>>> {
        let ressource_path = self.base_path.join(db);

        // We do a read_dir for now, it would be better to instead have an index IMO
        let mut read_dir = tokio::fs::read_dir(ressource_path).await?;
        let a = async_stream::stream! {
            while let Ok(Some(entry)) = read_dir.next_entry().await {
                let name = entry
                    .file_name()
                    .into_string()
                    .map_err(|_err| anyhow::anyhow!("Couldn't convert OsString to String."))?;

                if name.ends_with(".meta") {
                    yield Ok::<_, anyhow::Error>(name);
                }
            }
        };

        Ok(Box::new(a))
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
