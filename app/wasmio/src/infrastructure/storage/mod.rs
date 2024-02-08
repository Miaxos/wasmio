use std::path::PathBuf;

/// Implement this trait which define the backend storage used to store data
pub trait BackendStorage {}

#[derive(Debug, Clone)]
pub struct FSStorage {
    base_path: PathBuf,
}

impl FSStorage {
    pub fn new(base: PathBuf) -> Self {
        Self { base_path: base }
    }
}

impl BackendStorage for FSStorage {}
