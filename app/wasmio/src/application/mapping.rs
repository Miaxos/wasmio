use axum::routing::get;
use axum::Router;

use super::state::AppState;

pub struct AppMapping {
    state: AppState,
}

impl AppMapping {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub fn into_router(self) -> Router {
        Router::new()
            .with_state(self.state)
            .route("/", get(handler))
    }
}

async fn handler() -> &'static str {
    "hi mom!"
}
