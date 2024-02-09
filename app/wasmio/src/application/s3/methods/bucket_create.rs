use axum::body::Body;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::Response;
use axum_serde::xml::Xml;
use tracing::info;
use wasmio_aws_types::types::CreateBucketConfiguration;

use crate::application::s3::axum::request_context::RequestContext;
use crate::application::s3::errors::S3HTTPError;
use crate::application::s3::state::S3State;
use crate::domain::storage::BackendDriver;

pub async fn bucket_create_handle<T: BackendDriver>(
    req: RequestContext,
    State(state): State<S3State<T>>,
    Xml(_input): Xml<CreateBucketConfiguration>,
) -> Result<Response, S3HTTPError> {
    let bucket_name = req.bucket();

    info!(message = "Creating a new bucket", bucket = %bucket_name);

    state
        .bucket_loader
        .create_new_bucket(bucket_name)
        .await
        .map_err(|err| req.to_error_code(err))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::LOCATION, format!("/{bucket_name}"))
        .body(Body::empty())
        .unwrap())
}
