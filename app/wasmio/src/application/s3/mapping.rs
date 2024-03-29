use axum::body::Body;
use axum::error_handling::HandleError;
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::Router;
use tower::ServiceBuilder;

use super::context::{Context, S3Handler, VisitorNil};
use super::errors::S3HTTPError;
use super::handlers::bucket_create::BucketCreateHandler;
use super::handlers::object_delete::ObjectDeleteHandler;
use super::handlers::object_get::ObjectGetHandler;
use super::handlers::object_list_v2::ObjectListHandlerV2;
use super::handlers::object_put::ObjectPutHandler;
use super::state::S3State;
use crate::domain::storage::errors::BucketStorageError;
use crate::domain::storage::BackendDriver;
use crate::infrastructure::storage::BackendStorage;

pub struct S3Mapping<T: BackendDriver> {
    state: S3State<T>,
}

async fn handle_error(err: S3HTTPError) -> Response {
    err.into_response()
}

impl<T: BackendDriver> S3Mapping<T>
where
    BucketStorageError: From<<T as BackendStorage>::Error>,
{
    pub fn new(state: S3State<T>) -> Self {
        Self { state }
    }

    pub fn into_router(self) -> Router {
        let handlers = VisitorNil
            .with(BucketCreateHandler)
            .with(ObjectPutHandler)
            .with(ObjectDeleteHandler)
            .with(ObjectListHandlerV2)
            .with(ObjectGetHandler);

        let service =
            ServiceBuilder::new().service_fn(move |req: Request<Body>| {
                let state = self.state.clone();
                // Create a request context and route it based on this.
                async move {
                    let context = Context::new(req)?;
                    let r_id = context.request_id();
                    let resource = context.resource();

                    handlers.handle(context, state).await.map_err(|err| {
                        S3HTTPError::custom(resource, r_id.to_string(), err)
                    })
                }
            });

        let handle = HandleError::new(service, handle_error);
        Router::new().fallback_service(handle)
    }
}
