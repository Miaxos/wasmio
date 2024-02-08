use axum::{
    body::Body,
    extract::{Host, State},
    http::{header, StatusCode},
    response::Response,
};
use axum_serde::xml::Xml;
use serde_aws_types::types::create_bucket_configuration::CreateBucketConfiguration;
use tracing::info;

use crate::application::s3::{
    errors::{S3ErrorCodeKind, S3HTTPError},
    state::S3State,
};

pub async fn bucket_create_handle(
    Host(host): Host,
    State(_state): State<S3State>,
    Xml(_input): Xml<CreateBucketConfiguration>,
) -> Result<Response, S3HTTPError> {
    let bucket_name = match host.split_once('.') {
        Some((bucket_name, _)) => bucket_name,
        _ => {
            return Err(S3HTTPError::custom(
                "",
                "",
                S3ErrorCodeKind::InvalidBucketName,
            ));
        }
    };

    info!(message = "Creating a new bucket", bucket = %bucket_name);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::LOCATION, format!("/{bucket_name}"))
        .body(Body::empty())
        .unwrap())
}
