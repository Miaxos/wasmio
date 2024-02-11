#![allow(dead_code)]
use std::collections::HashMap;
use std::pin::Pin;

use axum::async_trait;
use futures::Stream;
use tokio::io::{AsyncRead, AsyncWrite};

mod fs_storage;
pub use fs_storage::{FSError, FSStorage};

mod database;
pub use database::DatabaseInfo;

mod element;
pub use element::ElementInfo;

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
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ElementInfo, Self::Error>> + Send>>,
        Self::Error,
    >;

    /// Get element from the database,
    async fn get_element_in_database<T: AsyncWrite + Send + Unpin>(
        &self,
        db: &str,
        key: &str,
        writer: &mut T,
    ) -> Result<u64, Self::Error>;

    /// Get element from the database,
    async fn get_element_metadata_in_database(
        &self,
        db: &str,
        key: &str,
    ) -> Result<Option<ElementInfo>, Self::Error>;

    /// Put an element inside database
    async fn insert_element_in_database<R: AsyncRead + Unpin + Send>(
        &self,
        db: &str,
        name_elt: &str,
        metadatas: HashMap<String, String>,
        content: &mut R,
    ) -> Result<ElementInfo, Self::Error>;

    /// Put an element inside database
    async fn delete_element_in_database(
        &self,
        db: &str,
        name_elt: &str,
    ) -> Result<(), Self::Error>;
}
