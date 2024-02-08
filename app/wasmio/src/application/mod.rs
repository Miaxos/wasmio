use std::net::SocketAddr;

use tokio::task::JoinHandle;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::info;

mod state;
use state::AppState;

mod mapping;
use mapping::AppMapping;

mod s3;

#[derive(Debug)]
pub struct Application {
    state: AppState,
}

impl Application {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    /// TODO: Proper shutdown process
    pub fn serve(self, addr: SocketAddr) -> JoinHandle<anyhow::Result<()>> {
        let app = AppMapping::new(self.state);
        let router = app.into_router().layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

        tokio::spawn(async move {
            info!("Server starting at {addr}");
            axum::Server::bind(&addr)
                .serve(router.into_make_service_with_connect_info::<SocketAddr>())
                .await?;

            Ok::<(), anyhow::Error>(())
        })
    }
}
