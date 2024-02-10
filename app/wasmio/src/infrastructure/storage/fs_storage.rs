#![allow(dead_code)]
use std::path::PathBuf;
use std::pin::Pin;

use axum::async_trait;
use base64ct::{Base64, Encoding};
use chrono::Utc;
use futures::future::join;
use futures::{Stream, TryStreamExt};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::warn;

use super::{BackendStorage, DatabaseInfo, ElementInfo};

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

    async fn database_metadata(
        &self,
        name: &str,
    ) -> anyhow::Result<Option<DatabaseInfo>> {
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

        let ressource_path =
            self.base_path.join(db).join(format!("{name_elt}.0.part.0"));
        let mut file_content = tokio::fs::File::create(ressource_path).await?;

        // TODO: test based on `cat public/data/test-bucket/test.txt.0.part.0 |
        // openssl sha256 -binary | base64`
        let mut hasher = Sha256::new();

        let stream = tokio_util::io::ReaderStream::new(content);
        let mut ar = tokio_util::io::StreamReader::new(stream.map_ok(|x| {
            hasher.update(&*x);
            x
        }));

        let size = tokio::io::copy(&mut ar, &mut file_content).await?;
        let hash = Base64::encode_string(&hasher.finalize());

        let metadata_path =
            self.base_path.join(db).join(format!("{name_elt}.0.meta"));
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

    async fn get_element_in_database<T: AsyncWrite + Send + Unpin>(
        &self,
        db: &str,
        key: &str,
        mut writer: &mut T,
    ) -> anyhow::Result<u64> {
        let ressource_path =
            self.base_path.join(db).join(format!("{key}.0.part.0"));
        let mut file_content = tokio::fs::File::open(ressource_path).await?;

        let size = tokio::io::copy(&mut file_content, &mut writer).await?;

        Ok(size)
    }

    async fn list_element_in_database(
        &self,
        db: &str,
        _start_after: Option<&str>,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = anyhow::Result<String>>>>>
    {
        let ressource_path = self.base_path.join(db);

        // We do a read_dir for now, it would be better to instead have an index
        // IMO
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

        Ok(Box::pin(a))
    }

    async fn delete_element_in_database(
        &self,
        db: &str,
        key: &str,
    ) -> anyhow::Result<()> {
        let ressource_path =
            self.base_path.join(db).join(format!("{key}.0.part.0"));
        let metadata_path =
            self.base_path.join(db).join(format!("{key}.0.meta"));

        let a = tokio::fs::remove_file(ressource_path);
        let b = tokio::fs::remove_file(metadata_path);
        let (a, b) = join(a, b).await;
        a?;
        b?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn simple_db_with_fs() {
        let temp = tempdir().expect("Failed to create temporary directory");
        let fs = FSStorage::new(temp.path().to_path_buf());

        let result = fs.new_database("test_db").await;
        assert!(result.is_ok());

        let check_metadata_info = fs.database_metadata("test_db").await;
        assert!(check_metadata_info.is_ok());
        let check_metadata_info = check_metadata_info.unwrap();
        assert!(check_metadata_info.is_some());
    }

    #[tokio::test]
    async fn test_new_database() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let storage = FSStorage::new(temp_dir.path().to_path_buf());

        let db_name = "test_db";
        let db_info = storage.new_database(db_name).await.unwrap();

        assert_eq!(db_info.name, db_name);
        assert_eq!(db_info.number_element, 0);

        let metadata_path = temp_dir.path().join(format!("{}.meta", db_name));
        assert!(metadata_path.exists());
    }

    #[tokio::test]
    async fn test_database_metadata() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let storage = FSStorage::new(temp_dir.path().to_path_buf());

        let db_name = "test_db";
        let db_info = storage.new_database(db_name).await.unwrap();

        let retrieved_db_info =
            storage.database_metadata(db_name).await.unwrap().unwrap();
        assert_eq!(db_info, retrieved_db_info);
    }

    #[tokio::test]
    async fn test_insert_and_get_element_in_database() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let storage = FSStorage::new(temp_dir.path().to_path_buf());

        let db_name = "test_db";
        storage.new_database(db_name).await.unwrap();

        let element_name = "test_element";
        let element_content = b"test_content";

        let mut element_reader = std::io::Cursor::new(element_content);

        let element_info = storage
            .insert_element_in_database(
                db_name,
                element_name,
                &mut element_reader,
            )
            .await
            .unwrap();

        let mut retrieved_content = Vec::new();
        let size = storage
            .get_element_in_database(
                db_name,
                element_name,
                &mut retrieved_content,
            )
            .await
            .unwrap();

        assert_eq!(size, element_content.len() as u64);
        assert_eq!(retrieved_content, element_content);

        assert_eq!(element_info.name, element_name);
        assert_eq!(element_info.size, size);
    }

    #[tokio::test]
    async fn test_list_element_in_database() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let storage = FSStorage::new(temp_dir.path().to_path_buf());

        let db_name = "test_db";
        storage.new_database(db_name).await.unwrap();

        let element_name = "test_element";
        storage
            .insert_element_in_database(
                db_name,
                element_name,
                &mut std::io::Cursor::new(b""),
            )
            .await
            .unwrap();

        let mut element_list_stream = storage
            .list_element_in_database(db_name, None)
            .await
            .unwrap();

        let mut found_element = false;
        while let Some(result) = element_list_stream.next().await {
            match result {
                Ok(name) => {
                    if name == format!("{}.0.meta", element_name) {
                        found_element = true;
                        break;
                    }
                }
                Err(_) => {
                    panic!("Error occurred while listing elements.");
                }
            }
        }

        assert!(found_element);
    }

    #[tokio::test]
    async fn test_delete_element_in_database() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let storage = FSStorage::new(temp_dir.path().to_path_buf());

        let db_name = "test_db";
        storage.new_database(db_name).await.unwrap();

        let element_name = "test_element";
        storage
            .insert_element_in_database(
                db_name,
                element_name,
                &mut std::io::Cursor::new(b""),
            )
            .await
            .unwrap();

        let delete_result = storage
            .delete_element_in_database(db_name, element_name)
            .await;

        assert!(delete_result.is_ok());

        let mut element_list_stream = storage
            .list_element_in_database(db_name, None)
            .await
            .unwrap();

        let mut found_element = false;
        while let Some(result) = element_list_stream.next().await {
            match result {
                Ok(name) => {
                    if name == format!("{}.0.meta", element_name) {
                        found_element = true;
                        break;
                    }
                }
                Err(_) => {
                    panic!("Error occurred while listing elements.");
                }
            }
        }

        assert!(!found_element);
    }

    #[tokio::test]
    async fn test_list_empty_database() {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let storage = FSStorage::new(temp_dir.path().to_path_buf());

        let db_name = "empty_db";
        storage.new_database(db_name).await.unwrap();

        let mut element_list_stream = storage
            .list_element_in_database(db_name, None)
            .await
            .unwrap();

        assert!(element_list_stream.next().await.is_none());
    }
}
