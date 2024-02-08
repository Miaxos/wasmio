use crate::domain::storage::BackendDriver;
use axum::{
    body::Body,
    extract::{Host, State},
    http::{header, StatusCode},
    response::Response,
};
use axum_serde::xml::Xml;
use serde_aws_types::types::create_bucket_configuration::CreateBucketConfiguration;
use tracing::{info, Instrument};

use crate::application::s3::{
    errors::{S3ErrorCodeKind, S3HTTPError},
    state::S3State,
};

pub async fn bucket_create_handle<T: BackendDriver>(
    Host(host): Host,
    State(state): State<S3State<T>>,
    Xml(_input): Xml<CreateBucketConfiguration>,
) -> Result<Response, S3HTTPError> {
    // TODO: We should ensure the form of host is `bucket.domain.io` with domain set in
    // configuration
    let bucket_name = match host.split_once('.') {
        Some((bucket_name, _)) => bucket_name,
        _ => {
            return Err(S3HTTPError::custom(
                &host,
                "42",
                S3ErrorCodeKind::InvalidBucketName,
            ));
        }
    };

    info!(message = "Creating a new bucket", bucket = %bucket_name);

    state
        .bucket_loader
        .create_new_bucket(bucket_name)
        .await
        .map_err(|err| S3HTTPError::custom(&host, "42", S3ErrorCodeKind::from(err)))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::LOCATION, format!("/{bucket_name}"))
        .body(Body::empty())
        .unwrap())
}
