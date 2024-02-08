use axum::routing::get;
use axum::Router;

use super::s3::mapping::S3Mapping;
use super::s3::state::S3State;
use super::state::AppState;

pub struct AppMapping {
    state: AppState,
}

impl AppMapping {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub fn into_router(self) -> Router {
        let s3mapping = S3Mapping::new(S3State::from_state(&self.state));

        Router::new()
            .with_state(self.state)
            .route("/", get(handler))
            .merge(s3mapping.into_router())
    }
}

async fn handler() -> &'static str {
    "hi mom!"
}
