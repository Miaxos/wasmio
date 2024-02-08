use axum::{
    body::{Body, HttpBody},
    extract::{Host, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use axum_serde::xml::Xml;
use serde_aws_types::types::create_bucket_configuration::CreateBucketConfiguration;
use tracing::info;

use crate::application::s3::state::S3State;

pub async fn bucket_create_handle(
    Host(host): Host,
    State(state): State<S3State>,
    Xml(input): Xml<CreateBucketConfiguration>,
) -> impl IntoResponse {
    let bucket_name = host.split_once('.');

    Response::builder()
        .status(StatusCode::OK)
        .header(header::LOCATION, "wasmio")
        .body(Body::empty())
        .unwrap()
}
