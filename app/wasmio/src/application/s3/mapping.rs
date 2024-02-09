use axum::routing::put;
use axum::Router;

use crate::domain::storage::BackendDriver;

use super::methods::bucket_create::bucket_create_handle;
use super::methods::object_put::object_put_handle;
use super::state::S3State;

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
            .with_state(self.state)
    }
}
