use axum::routing::{delete, put};
use axum::Router;

use super::methods::bucket_create::bucket_create_handle;
use super::methods::object_delete::object_delete_handle;
use super::methods::object_put::object_put_handle;
use super::state::S3State;
use crate::domain::storage::BackendDriver;

pub struct S3Mapping<T: BackendDriver> {
    state: S3State<T>,
}

impl<T: BackendDriver> S3Mapping<T> {
    pub fn new(state: S3State<T>) -> Self {
        Self { state }
    }

    pub fn into_router(self) -> Router {
        Router::new()
            .route("/", put(bucket_create_handle))
            .route("/:key", put(object_put_handle))
            .route("/:key", delete(object_delete_handle))
            .with_state(self.state)
    }
}
