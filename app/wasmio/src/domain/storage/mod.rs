use std::fmt::Debug;

use crate::infrastructure::storage::{BackendStorage, ElementInfo, FSStorage};

pub mod errors;
use axum::body::{Body, BodyDataStream};
use errors::BucketStorageError;
use futures::{StreamExt, TryStreamExt};
use tokio_util::io::{ReaderStream, StreamReader};
use tracing::{error, warn};
use wasmio_aws_types::types::{
    CreateBucketOutput, CreateBucketOutputBuilder, CreateBucketRequest,
    DeleteObjectOutput, DeleteObjectOutputBuilder, DeleteObjectRequest,
    GetObjectOutput, GetObjectRequest, ListObjectsV2Output,
    ListObjectsV2Request, Object, PutObjectOutput, PutObjectOutputBuilder,
    PutObjectRequest,
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
            bucket,
            key,
            body,
            metadata,
            ..
        }: PutObjectRequest,
    ) -> Result<PutObjectOutput, BucketStorageError> {
        let body = body.ok_or(BucketStorageError::Unknown)?;
        let body_err = body
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
        let mut body_reader = StreamReader::new(body_err);

        self.backend_storage
            .insert_element_in_database(
                &bucket,
                &key,
                metadata.unwrap_or_default(),
                &mut body_reader,
            )
            .await?;

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
                Ok(ElementInfo {
                    name,
                    last_modified,
                    size,
                    checksum,
                    ..
                }) => {
                    contents.push(Object {
                        key: Some(name),
                        size: Some(size as i64),
                        last_modified: Some(last_modified.to_rfc3339()),
                        e_tag: Some(checksum),
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

    pub async fn get_object(
        &self,
        GetObjectRequest { bucket, key, .. }: GetObjectRequest,
    ) -> Result<GetObjectOutput, BucketStorageError> {
        // TODO: Ensure the file exist before reading it, right now, the error
        // is not bubbled up as we do an async read.
        let (mut asyncwriter, asyncreader) = tokio::io::duplex(8192);

        // Ugly shit
        let s = self.clone();
        let b = bucket.clone();
        let k = key.clone();
        tokio::spawn(async move {
            if let Err(err) = s
                .backend_storage
                .get_element_in_database(&b, &k, &mut asyncwriter)
                .await
            {
                warn!("{err:?}");
            }
        });

        let ElementInfo {
            size,
            last_modified,
            checksum,
            metadatas,
            ..
        } = match self
            .backend_storage
            .get_element_metadata_in_database(&bucket, &key)
            .await?
        {
            Some(elt) => elt,
            None => {
                return Err(BucketStorageError::NoKey);
            }
        };

        let body = Body::from_stream(ReaderStream::new(asyncreader));

        Ok(GetObjectOutput {
            accept_ranges: None,
            body: Some(body),
            bucket_key_enabled: None,
            cache_control: None,
            content_disposition: None,
            content_encoding: None,
            content_language: None,
            content_length: Some(size as i64),
            content_range: None,
            content_type: None,
            delete_marker: None,
            e_tag: Some(checksum),
            expiration: None,
            expires: None,
            last_modified: Some(last_modified.to_rfc3339()),
            metadata: Some(metadatas),
            missing_meta: None,
            object_lock_legal_hold_status: None,
            object_lock_mode: None,
            object_lock_retain_until_date: None,
            parts_count: None,
            replication_status: None,
            request_charged: None,
            restore: None,
            sse_customer_algorithm: None,
            sse_customer_key_md5: None,
            ssekms_key_id: None,
            server_side_encryption: None,
            storage_class: None,
            tag_count: None,
            version_id: None,
            website_redirect_location: None,
        })
    }
}
