use crate::application::state::AppState;
use crate::domain::storage::{BackendDriver, BucketStorage};
use crate::infrastructure::storage::FSStorage;

#[derive(Debug, Clone)]
pub struct S3State<T: BackendDriver> {
    pub bucket_loader: BucketStorage<T>,
}

impl S3State<FSStorage> {
    pub fn from_state(app: &AppState) -> Self {
        Self {
            bucket_loader: BucketStorage::new(app.storage.clone()),
        }
    }
}
