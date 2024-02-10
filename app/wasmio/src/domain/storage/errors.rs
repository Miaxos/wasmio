#[derive(Clone, Debug, thiserror::Error)]
pub enum BucketStorageError {
    #[error("An issue happened")]
    Unknown,
}
