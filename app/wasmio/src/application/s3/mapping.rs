use axum::body::Body;
use axum::error_handling::HandleError;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::Router;
use tower::ServiceBuilder;
use ulid::Ulid;

use super::context::{Context, S3Handler, VisitorNil};
use super::errors::S3HTTPError;
use super::handlers::bucket_create::BucketCreateHandler;
use super::handlers::object_put::ObjectPutHandler;
use super::state::S3State;
use crate::domain::storage::BackendDriver;

pub struct S3Mapping<T: BackendDriver> {
    state: S3State<T>,
}

async fn handle_error(err: S3HTTPError) -> Response {
    err.into_response()
}

impl<T: BackendDriver> S3Mapping<T> {
    pub fn new(state: S3State<T>) -> Self {
        Self { state }
    }

    pub fn into_router(self) -> Router {
        let handlers =
            VisitorNil.with(BucketCreateHandler).with(ObjectPutHandler);

        let service =
            ServiceBuilder::new().service_fn(move |req: Request<Body>| {
                let request_id = Ulid::new();
                let state = self.state.clone();
                // Create a request context and route it based on this.
                async move {
                    let result = handlers
                        .handle(Context::new(req), state)
                        .await
                        .map_err(|err| {
                            S3HTTPError::custom("", request_id.to_string(), err)
                        });

                    result
                }
            });

        let handle = HandleError::new(service, handle_error);
        Router::new().fallback_service(handle)
    }
}
