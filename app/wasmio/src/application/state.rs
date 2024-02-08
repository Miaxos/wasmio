use crate::infrastructure::storage::FSStorage;

#[derive(Debug, Clone)]
pub struct AppState {
    pub storage: FSStorage,
}

impl AppState {
    pub fn new(storage: FSStorage) -> Self {
        Self { storage }
    }
}
