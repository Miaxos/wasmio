use axum::routing::put;
use axum::Router;

use super::methods::bucket_create::bucket_create_handle;
use super::state::S3State;

pub struct S3Mapping {
    state: S3State,
}

impl S3Mapping {
    pub fn new(state: S3State) -> Self {
        Self { state }
    }

    pub fn into_router(self) -> Router {
        Router::new()
            .route("/", put(bucket_create_handle))
            .with_state(self.state)
    }
}
