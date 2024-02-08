use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use tracing::error;

/// S3 partiel error code enum
///
/// See [`ErrorResponses`](https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html)
#[derive(Clone, Copy, strum::Display)]
#[allow(clippy::upper_case_acronyms)]
#[non_exhaustive]
pub enum S3ErrorCodeKind {
    /// The specified bucket is not valid.
    InvalidBucketName,
}

impl S3ErrorCodeKind {
    fn status_code(&self) -> StatusCode {
        match self {
            S3ErrorCodeKind::InvalidBucketName => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            S3ErrorCodeKind::InvalidBucketName => "The specified bucket is not valid.",
        }
    }
}

pub struct S3HTTPError {
    /// The bucket or object that is involved in the error.
    ressource: String,
    /// ID of the request associated with the error.
    request_id: String,
    kind: S3ErrorCodeKind,
}

impl S3HTTPError {
    pub fn custom(ressource: &str, request_id: &str, kind: S3ErrorCodeKind) -> Self {
        Self {
            // TODO: little ugly but it's to have a proper impl quickly
            request_id: request_id.to_string(),
            ressource: ressource.to_string(),
            kind,
        }
    }
}

// ------------------------------------------------------------------------

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
