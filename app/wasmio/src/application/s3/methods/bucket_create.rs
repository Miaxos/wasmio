use crate::{
    application::s3::axum::request_context::RequestContext, domain::storage::BackendDriver,
};
use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode},
    response::Response,
};
use axum_serde::xml::Xml;
use serde_aws_types::types::CreateBucketConfiguration;
use tracing::info;

use crate::application::s3::{errors::S3HTTPError, state::S3State};

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
        .map_err(|err| req.from_error_code(err))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::LOCATION, format!("/{bucket_name}"))
        .body(Body::empty())
        .unwrap())
}
