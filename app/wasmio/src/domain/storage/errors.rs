use crate::infrastructure::storage::FSError;

#[derive(Clone, Debug, thiserror::Error)]
pub enum BucketStorageError {
    #[error("An issue happened")]
    Unknown,
    #[error("Database already exist")]
    DatabaseAlreadyExist,
    #[error("No bucket")]
    NoBucket,
    #[error("No key")]
    NoKey,
}

impl From<FSError> for BucketStorageError {
    fn from(value: FSError) -> Self {
        match value {
            FSError::AlreadyExist => Self::DatabaseAlreadyExist,
            FSError::NoDatabase => Self::NoBucket,
            _ => Self::Unknown,
        }
    }
}
