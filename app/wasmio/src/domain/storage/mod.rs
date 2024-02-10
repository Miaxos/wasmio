use std::fmt::Debug;

use crate::infrastructure::storage::{BackendStorage, FSStorage};

pub mod errors;
use errors::BucketStorageError;
use futures::{StreamExt, TryStreamExt};
use tokio_util::io::StreamReader;
use tracing::{error, warn};
use wasmio_aws_types::types::{
    CreateBucketOutput, CreateBucketOutputBuilder, CreateBucketRequest,
    DeleteObjectOutput, DeleteObjectOutputBuilder, DeleteObjectRequest,
    ListObjectsV2Output, ListObjectsV2Request, Object, PutObjectOutput,
    PutObjectOutputBuilder, PutObjectRequest,
};

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

impl<T> BucketStorage<T>
where
    T: BackendDriver,
    BucketStorageError: From<<T as BackendStorage>::Error>,
{
    pub fn new(storage: T) -> Self {
        Self {
            backend_storage: storage,
        }
    }

    pub async fn create_new_bucket(
        &self,
        CreateBucketRequest { bucket, .. }: CreateBucketRequest,
    ) -> Result<CreateBucketOutput, BucketStorageError> {
        let db_info = self.backend_storage.new_database(&bucket).await?;

        Ok(CreateBucketOutputBuilder::default()
            .location(format!("/{name}", name = db_info.name()))
            .build()
            .map_err(|_err| BucketStorageError::Unknown)?)
    }

    pub async fn put_object(
        &self,
        PutObjectRequest {
            bucket, key, body, ..
        }: PutObjectRequest,
    ) -> Result<PutObjectOutput, BucketStorageError> {
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

        Ok(PutObjectOutputBuilder::default()
            .e_tag(Some("unimplemented".to_string()))
            .build()
            .map_err(|_err| BucketStorageError::Unknown)?)
    }

    pub async fn delete_object(
        &self,
        DeleteObjectRequest { bucket, key, .. }: DeleteObjectRequest,
    ) -> Result<DeleteObjectOutput, BucketStorageError> {
        self.backend_storage
            .delete_element_in_database(&bucket, &key)
            .await
            .map_err(|err| {
                error!("{err:?}");
                BucketStorageError::Unknown
            })?;

        Ok(DeleteObjectOutputBuilder::default()
            .build()
            .map_err(|_err| BucketStorageError::Unknown)?)
    }

    pub async fn list_object_v2(
        &self,
        ListObjectsV2Request { bucket, .. }: ListObjectsV2Request,
    ) -> Result<ListObjectsV2Output, BucketStorageError> {
        let mut s = self
            .backend_storage
            .list_element_in_database(&bucket, None)
            .await
            .map_err(|_| BucketStorageError::Unknown)?;

        let mut contents: Vec<Object> = Vec::new();
        while let Some(elt) = s.next().await {
            match elt {
                Ok(elt) => {
                    contents.push(Object {
                        key: Some(elt),
                        ..Default::default()
                    });
                }
                Err(err) => {
                    warn!("{err:?}");
                }
            }
        }

        let result = ListObjectsV2Output {
            contents: Some(contents),
            ..Default::default()
        };
        Ok(result)
    }
}
