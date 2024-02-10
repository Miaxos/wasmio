#![allow(dead_code)]
use std::pin::Pin;

use axum::async_trait;
use chrono::{DateTime, Utc};
use futures::Stream;
use tokio::io::{AsyncRead, AsyncWrite};

mod fs_storage;
pub use fs_storage::{FSError, FSStorage};

mod database;
pub use database::DatabaseInfo;

/// Implement this trait which define the backend storage used to store data
///
/// The storage is very simple for now
///
/// It supports creating a Database
///
/// - Storing elements in the database
/// - Storing metadata in the database, the idea for metadata is: it should be
///   fast to manipulate and it'll allow us to create index based on those
///   metadata.
#[async_trait]
pub trait BackendStorage: Send + Sync {
    type Error: std::error::Error + Send + Sync;

    /// To create a new database
    async fn new_database(
        &self,
        name: &str,
    ) -> Result<DatabaseInfo, Self::Error>;

    /// To get database metadata, if None, database doesn't exist
    async fn database_metadata(
        &self,
        name: &str,
    ) -> Result<Option<DatabaseInfo>, Self::Error>;

    /// List elements from the database,
    async fn list_element_in_database(
        &self,
        db: &str,
        start_after: Option<&str>,
    ) -> anyhow::Result<
        Pin<Box<dyn Stream<Item = anyhow::Result<String>> + Send>>,
    >;

    /// Get element from the database,
    async fn get_element_in_database<T: AsyncWrite + Send + Unpin>(
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
    ) -> Result<ElementInfo, Self::Error>;

    /// Put an element inside database
    async fn delete_element_in_database(
        &self,
        db: &str,
        name_elt: &str,
    ) -> anyhow::Result<()>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ElementInfo {
    pub name: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    /// Only using sha256 for now
    pub checksum: String,
}
