use std::fmt::Display;

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::domain::storage::errors::BucketStorageError;

/// S3 partiel error code enum
///
/// See [`ErrorResponses`](https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html)
#[derive(Debug, Clone, Copy, strum::Display)]
#[allow(clippy::upper_case_acronyms)]
#[non_exhaustive]
pub enum S3ErrorCodeKind {
    /// The specified bucket is not valid.
    InvalidBucketName,
    /// An internal error occurred. Try again.
    InternalError,
}

impl S3ErrorCodeKind {
    fn status_code(&self) -> StatusCode {
        match self {
            S3ErrorCodeKind::InvalidBucketName => StatusCode::BAD_REQUEST,
            S3ErrorCodeKind::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            S3ErrorCodeKind::InvalidBucketName => "The specified bucket is not valid.",
            S3ErrorCodeKind::InternalError => "An internal error occurred. Try again.",
        }
    }
}

#[derive(Debug)]
pub struct S3HTTPError {
    /// The bucket or object that is involved in the error.
    ressource: String,
    /// ID of the request associated with the error.
    request_id: String,
    kind: S3ErrorCodeKind,
}

impl Display for S3HTTPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl std::error::Error for S3HTTPError {}

impl S3HTTPError {
    pub fn custom<S1: AsRef<str>, S2: AsRef<str>>(
        ressource: S1,
        request_id: S2,
        kind: S3ErrorCodeKind,
    ) -> Self {
        Self {
            // TODO: little ugly but it's to have a proper impl quickly
            request_id: request_id.as_ref().to_string(),
            ressource: ressource.as_ref().to_string(),
            kind,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub message: &'static str,
    pub ressource: String,
    pub request_id: String,
}

impl IntoResponse for S3HTTPError {
    fn into_response(self) -> axum::response::Response {
        let err = match quick_xml::se::to_string(&Error {
            code: self.kind.to_string(),
            message: self.kind.message(),
            ressource: self.ressource,
            request_id: self.request_id,
        }) {
            Ok(elt) => elt,
            Err(err) => {
                error!("{err:?}");
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::new("wtf".to_string()))
                    .unwrap();
            }
        };

        let body = format!(
            r###"<?xml version="1.0" encoding="UTF-8"?>
{err}
"###,
            err = err
        );

        Response::builder()
            .status(self.kind.status_code())
            .header(axum::http::header::CONTENT_TYPE, "application/xml")
            .body(Body::new(body))
            .unwrap()
    }
}

// ----------------------------------------------------------------------------

impl From<BucketStorageError> for S3ErrorCodeKind {
    fn from(value: BucketStorageError) -> Self {
        match value {
            BucketStorageError::Unknown => Self::InternalError,
        }
    }
}
