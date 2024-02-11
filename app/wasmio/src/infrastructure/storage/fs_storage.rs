#![allow(dead_code)]
use std::collections::HashMap;
use std::io::ErrorKind;
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use std::pin::Pin;

use axum::async_trait;
use base64ct::{Base64, Encoding};
use chrono::Utc;
use futures::future::join;
use futures::{Stream, TryStreamExt};
use libc::c_int;
use sha2::{Digest, Sha256};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::warn;

use super::{BackendStorage, DatabaseInfo, ElementInfo};

pub const LOCK_EX: c_int = 2;
pub const LOCK_UN: c_int = 8;
extern "C" {
    fn flock(fd: c_int, operation: c_int) -> c_int;
}

/// We have a FSStorage implemented which aims to store files inside the FS.
///
/// A database is composed of multiple files:
///   - A Folder where every elements are going to be inside
///   - A `.meta` which will give us a fast path to retrieve data on a database
///
/// An element is compose of multiples files:
///   - Files representing the content, splitted into multiple parts (right now
///     no multipart, so no
///   splitting).
///   - A `.meta` which contain the Info about the element, (meadatas)
#[derive(Debug, Clone)]
pub struct FSStorage {
    base_path: PathBuf,
}

impl FSStorage {
    pub fn new(base: PathBuf) -> Self {
        Self { base_path: base }
    }

    pub fn database_path(&self, db_name: &str) -> PathBuf {
        self.base_path.join(db_name)
    }

    pub fn database_path_meta(&self, db_name: &str) -> PathBuf {
        self.base_path.join(format!("{name}.meta", name = db_name))
    }

    pub fn database_path_lock(&self, db_name: &str) -> PathBuf {
        self.base_path.join(format!("{name}.lock", name = db_name))
    }

    // For now it's only one part, but later it could grow with versions too
    pub fn file_path(&self, db_name: &str, file_name: &str) -> PathBuf {
        self.base_path
            .join(db_name)
            .join(format!("{file_name}.part"))
    }

    pub fn file_path_lock(&self, db_name: &str, file_name: &str) -> PathBuf {
        self.base_path
            .join(db_name)
            .join(format!("{file_name}.lock"))
    }

    pub fn file_meta(&self, db_name: &str, file_name: &str) -> PathBuf {
        self.base_path
            .join(db_name)
            .join(format!("{file_name}.meta"))
    }

    pub async fn update_database(
        &self,
        db: DatabaseInfo,
    ) -> Result<(), <Self as BackendStorage>::Error> {
        tokio::fs::write(
            self.database_path_meta(db.name()),
            serde_json::to_string(&db)?,
        )
        .await?;
        Ok(())
    }

    pub async fn lock_for_write_db(
        &self,
        db: &str,
    ) -> Result<LockGuard, <Self as BackendStorage>::Error> {
        let path = self.database_path_lock(db);
        let file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .await?;
        unsafe { flock(file.as_raw_fd(), LOCK_EX) };
        Ok(LockGuard { file })
    }

    pub async fn lock_for_element(
        &self,
        db: &str,
        elt: &str,
    ) -> Result<LockGuard, <Self as BackendStorage>::Error> {
        let path = self.file_path_lock(db, elt);
        let file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .await?;
        unsafe { flock(file.as_raw_fd(), LOCK_EX) };
        Ok(LockGuard { file })
    }

    pub async fn load_file_metadata(
        &self,
        db_name: &str,
        file_name: &str,
    ) -> Result<Option<ElementInfo>, <Self as BackendStorage>::Error> {
        let metadata_path = self.file_meta(db_name, file_name);

        if (tokio::fs::metadata(&metadata_path).await).is_err() {
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(metadata_path).await?;
        let data_info = serde_json::from_str(&content)?;

        Ok(Some(data_info))
    }
}

pub struct LockGuard {
    file: File,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        unsafe { flock(self.file.as_raw_fd(), LOCK_UN) };
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FSError {
    // It's depending on the context in fact, will need to modify this
    #[error("Database already exist")]
    AlreadyExist,
    #[error("No database")]
    NoDatabase,
    #[error("fallback serde: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("IO: {0}")]
    Other(std::io::Error),
    #[error("weird one, to investigate")]
    Weird,
}

impl From<std::io::Error> for FSError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::AlreadyExists => Self::AlreadyExist,
            _ => Self::Other(value),
        }
    }
}

#[async_trait]
impl BackendStorage for FSStorage {
    type Error = FSError;

    async fn new_database(
        &self,
        name: &str,
    ) -> Result<DatabaseInfo, Self::Error> {
        let new_db = DatabaseInfo::new_database(name.to_string());

        let write_metadata = tokio::fs::write(
            self.database_path_meta(new_db.name()),
            serde_json::to_string(&new_db)?,
        );
        let write_dir =
            tokio::fs::create_dir(self.database_path(new_db.name()));

        let (write_metadata_task, write_dir_task) =
            join(write_metadata, write_dir).await;

        write_metadata_task?;
        write_dir_task?;

        Ok(new_db)
    }

    async fn database_metadata(
        &self,
        name: &str,
    ) -> Result<Option<DatabaseInfo>, Self::Error> {
        let ressource_path = self.database_path_meta(name);

        // We check the database exits
        if let Err(err) = tokio::fs::metadata(&ressource_path).await {
            warn!("{err:?}");
            return Ok(None);
        }

        // We load the DatabaseInfo
        let content = tokio::fs::read_to_string(ressource_path).await?;

        let data_info = serde_json::from_str(&content)?;
        Ok(Some(data_info))
    }

    async fn insert_element_in_database<R: AsyncRead + Unpin + Send>(
        &self,
        db: &str,
        name_elt: &str,
        metadatas: HashMap<String, String>,
        content: &mut R,
    ) -> Result<ElementInfo, Self::Error> {
        let now = Utc::now();
        let _lock = self.lock_for_element(db, name_elt).await?;

        if self.database_metadata(db).await?.is_none() {
            return Err(FSError::NoDatabase);
        }

        let ressource_path = self.file_path(db, name_elt);
        let metadata_path = self.file_meta(db, name_elt);

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

        let elt = self.load_file_metadata(db, name_elt).await?;
        let elt = ElementInfo {
            name: name_elt.to_string(),
            size,
            created_at: elt.map(|x| x.created_at).unwrap_or(now),
            last_modified: now,
            checksum: hash,
            metadatas,
        };
        tokio::fs::write(metadata_path, serde_json::to_string(&elt)?).await?;

        if let Some(DatabaseInfo {
            name,
            number_element,
            created_at,
        }) = self.database_metadata(db).await?
        {
            self.update_database(DatabaseInfo {
                name,
                number_element: number_element + 1,
                created_at,
            })
            .await?;
        }
        Ok(elt)
    }

    async fn get_element_metadata_in_database(
        &self,
        db: &str,
        key: &str,
    ) -> Result<Option<ElementInfo>, Self::Error> {
        let ressource_path = self.file_meta(db, key);

        if let Err(err) = tokio::fs::metadata(&ressource_path).await {
            warn!("{err:?}");
            return Ok(None);
        }

        let content = tokio::fs::read_to_string(ressource_path).await?;
        let data_info = serde_json::from_str(&content)?;
        Ok(Some(data_info))
    }

    async fn get_element_in_database<T: AsyncWrite + Send + Unpin>(
        &self,
        db: &str,
        key: &str,
        mut writer: &mut T,
    ) -> Result<u64, Self::Error> {
        let ressource_path = self.file_path(db, key);
        let metadata_path = self.file_meta(db, key);

        tokio::fs::metadata(&metadata_path).await?;
        let mut file_content = tokio::fs::File::open(ressource_path).await?;

        let size = tokio::io::copy(&mut file_content, &mut writer).await?;
        Ok(size)
    }

    async fn list_element_in_database(
        &self,
        db: &str,
        _start_after: Option<&str>,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ElementInfo, Self::Error>> + Send>>,
        Self::Error,
    > {
        let ressource_path = self.database_path(db);

        // We do a read_dir for now, it would be better to instead have an index
        // IMO
        let mut read_dir = tokio::fs::read_dir(ressource_path).await?;
        let a = async_stream::stream! {
            while let Ok(Some(entry)) = read_dir.next_entry().await {
                let name = entry
                    .file_name()
                    .into_string()
                    .map_err(|_err| FSError::Weird)?;

                if let Some(_name) = name.strip_suffix(".meta") {
                    let content = tokio::fs::read_to_string(entry.path()).await?;
                    let data_info = serde_json::from_str(&content)?;

                    yield Ok(data_info);
                }
            }
        };

        Ok(Box::pin(a))
    }

    async fn delete_element_in_database(
        &self,
        db: &str,
        key: &str,
    ) -> Result<(), Self::Error> {
        let _lock = self.lock_for_element(db, key).await?;
        let ressource_path = self.file_path(db, key);
        let metadata_path = self.file_meta(db, key);

        let a = tokio::fs::remove_file(ressource_path);
        let b = tokio::fs::remove_file(metadata_path);
        let (a, b) = join(a, b).await;
        a?;
        b?;

        if let Some(DatabaseInfo {
            name,
            number_element,
            created_at,
        }) = self.database_metadata(db).await?
        {
            self.update_database(DatabaseInfo {
                name,
                number_element: number_element - 1,
                created_at,
            })
            .await?;
        }

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
                Default::default(),
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
                Default::default(),
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
                    if name.name == element_name {
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
                Default::default(),
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
                    if name.name == element_name {
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
