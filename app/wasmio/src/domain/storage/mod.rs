use crate::infrastructure::storage::{BackendStorage, FSStorage};
use std::fmt::Debug;

pub mod errors;
use errors::BucketStorageError;

pub trait BackendDriver: BackendStorage + Debug + Send + Sync + Clone + 'static {}
impl BackendDriver for FSStorage {}

/// The [BucketStorage] is the struct shared in the application which allow you to access to some
/// [Bucket] and interact with those.
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

    pub async fn create_new_bucket(&self, bucket_name: &str) -> Result<(), BucketStorageError> {
        Ok(())
    }
}
