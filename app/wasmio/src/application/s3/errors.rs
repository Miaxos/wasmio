use std::fmt::Display;

use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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
    /// Couldn't parse the specified URI.
    InvalidURI,
    /// Your key is too long.
    KeyTooLongError,
    /// This error might occur for the following reasons:
    /// - The request is using the wrong signature version. Use
    ///   AWS4-HMAC-SHA256
    /// (Signature Version 4).
    /// - An access point can be created only for an existing bucket.
    /// - The access point is not in a state where it can be deleted.
    /// - An access point can be listed only for an existing bucket.
    /// - The next token is not valid.
    /// - At least one action must be specified in a lifecycle rule.
    /// - At least one lifecycle rule must be specified.
    /// - The number of lifecycle rules must not exceed the allowed limit of
    ///   1000 rules.
    /// - The range for the MaxResults parameter is not valid.
    /// - SOAP requests must be made over an HTTPS connection.
    /// - Amazon S3 Transfer Acceleration is not supported for buckets with
    /// non-DNS compliant names.
    /// - Amazon S3 Transfer Acceleration is not supported for buckets with
    /// periods (.) in their names.
    /// - The Amazon S3 Transfer Acceleration endpoint supports only virtual
    /// style requests.
    /// - Amazon S3 Transfer Acceleration is not configured on this bucket.
    /// - Amazon S3 Transfer Acceleration is disabled on this bucket.
    /// - Amazon S3 Transfer Acceleration is not supported on this bucket. For
    /// assistance, contact AWS Support.
    /// - Amazon S3 Transfer Acceleration cannot be enabled on this bucket. For
    /// assistance, contact AWS Support.
    /// - Conflicting values provided in HTTP headers and query parameters.
    /// - Conflicting values provided in HTTP headers and POST form fields.
    /// - CopyObject request made on objects larger than 5GB in size.
    InvalidRequest,
    /// This happens when the user sends malformed XML (XML that doesn't
    /// conform to the published XSD) for the configuration. The error message
    /// is, \"The XML you provided was not well-formed or did not validate
    /// against our published schema.\"
    MalformedXML,
}

impl S3ErrorCodeKind {
    const fn status_code(&self) -> StatusCode {
        match self {
            S3ErrorCodeKind::InvalidBucketName => StatusCode::BAD_REQUEST,
            S3ErrorCodeKind::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            S3ErrorCodeKind::InvalidRequest => StatusCode::BAD_REQUEST,
            S3ErrorCodeKind::InvalidURI => StatusCode::BAD_REQUEST,
            S3ErrorCodeKind::KeyTooLongError => StatusCode::BAD_REQUEST,
            S3ErrorCodeKind::MalformedXML => StatusCode::BAD_REQUEST,
        }
    }

    const fn message(&self) -> &'static str {
        match self {
            S3ErrorCodeKind::InvalidBucketName => {
                "The specified bucket is not valid."
            }
            S3ErrorCodeKind::InternalError => {
                "An internal error occurred. Try again."
            }
            S3ErrorCodeKind::InvalidRequest => "Invalid Request",
            S3ErrorCodeKind::InvalidURI => "Couldn't parse the specified URI.",
            S3ErrorCodeKind::KeyTooLongError => "Your key is too long",
            S3ErrorCodeKind::MalformedXML => {
                "The XML that you provided was not well formed or did not \
                 validate against our published schema."
            }
        }
    }
}

#[derive(Debug)]
pub struct S3Error {
    message: Option<String>,
    kind: S3ErrorCodeKind,
}

impl Display for S3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl S3Error {
    fn message(&self) -> &str {
        if let Some(msg) = &self.message {
            msg
        } else {
            self.kind.message()
        }
    }

    const fn status_code(&self) -> StatusCode {
        self.kind.status_code()
    }

    pub fn invalid_request(reason: &'static str) -> Self {
        Self {
            kind: S3ErrorCodeKind::InvalidRequest,
            message: Some(reason.to_string()),
        }
    }
}

impl From<S3ErrorCodeKind> for S3Error {
    fn from(value: S3ErrorCodeKind) -> Self {
        Self {
            kind: value,
            message: None,
        }
    }
}

#[derive(Debug)]
pub struct S3HTTPError {
    /// The bucket or object that is involved in the error.
    ressource: String,
    /// ID of the request associated with the error.
    request_id: String,
    kind: Box<S3Error>,
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
        kind: impl Into<S3Error>,
    ) -> Self {
        Self {
            // TODO: little ugly but it's to have a proper impl quickly
            request_id: request_id.as_ref().to_string(),
            ressource: ressource.as_ref().to_string(),
            kind: Box::new(kind.into()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub message: String,
    pub ressource: String,
    pub request_id: String,
}

impl IntoResponse for S3HTTPError {
    fn into_response(self) -> axum::response::Response {
        let err = match quick_xml::se::to_string(&Error {
            code: self.kind.to_string(),
            message: self.kind.message().to_string(),
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

impl From<BucketStorageError> for S3Error {
    fn from(value: BucketStorageError) -> Self {
        match value {
            BucketStorageError::Unknown => {
                S3ErrorCodeKind::InternalError.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;
    use http_body_util::BodyExt;

    use super::{S3ErrorCodeKind, S3HTTPError};

    #[tokio::test]
    async fn simple_response_error() {
        let response =
            S3HTTPError::custom("test", "blbl", S3ErrorCodeKind::InternalError)
                .into_response();

        insta::assert_debug_snapshot!(response);
        let body = response.into_body().collect().await.unwrap();

        let result = String::from_utf8(body.to_bytes().to_vec()).unwrap();
        insta::assert_display_snapshot!(result, @r###"
        <?xml version="1.0" encoding="UTF-8"?>
        <Error><code>InternalError</code><message>An internal error occurred. Try again.</message><ressource>test</ressource><request_id>blbl</request_id></Error>
        "###);
    }
}
