use std::fmt::Debug;

use crate::infrastructure::storage::{BackendStorage, FSStorage};

pub mod errors;
use errors::BucketStorageError;
use futures::TryStreamExt;
use serde_aws_types::types::{DeleteObjectRequest, PutObjectRequest};
use tokio_util::io::StreamReader;
use tracing::{error, warn};

pub trait BackendDriver:
    BackendStorage + Debug + Send + Sync + Clone + 'static
{
}
impl BackendDriver for FSStorage {}

/// The [BucketStorage] is the struct shared in the application which allow you
/// to access to some [Bucket] and interact with those.
///
/// For now, we implement the AWS S3 Api here as we have few methods, but it
/// might be interesting to split it by domain.
#[derive(Debug, Clone)]
pub struct BucketStorage<T: BackendDriver> {
    backend_storage: T,
}

impl<T: BackendDriver> BucketStorage<T> {
    pub fn new(storage: T) -> Self {
        Self {
            backend_storage: storage,
        }
    }

    pub async fn create_new_bucket(
        &self,
        bucket_name: &str,
    ) -> Result<(), BucketStorageError> {
        let _db_info = self
            .backend_storage
            .new_database(bucket_name)
            .await
            .map_err(|err| {
                warn!("{err:?}");
                BucketStorageError::Unknown
            })?;
        Ok(())
    }

    pub async fn put_object(
        &self,
        PutObjectRequest {
            bucket, key, body, ..
        }: PutObjectRequest,
    ) -> Result<(), BucketStorageError> {
        let body = body.ok_or(BucketStorageError::Unknown)?;
        let body_err = body
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
        let mut body_reader = StreamReader::new(body_err);

        self.backend_storage
            .insert_element_in_database(&bucket, &key, &mut body_reader)
            .await
            .map_err(|err| {
                error!("{err:?}");
                BucketStorageError::Unknown
            })?;

        Ok(())
    }

    pub async fn delete_object(
        &self,
        DeleteObjectRequest { bucket, key, .. }: DeleteObjectRequest,
    ) -> Result<(), BucketStorageError> {
        self.backend_storage
            .delete_element_in_database(&bucket, &key)
            .await
            .map_err(|err| {
                error!("{err:?}");
                BucketStorageError::Unknown
            })?;

        Ok(())
    }
}
