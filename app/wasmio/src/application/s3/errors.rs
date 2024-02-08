use axum::http::StatusCode;

/// S3 partiel error code enum
///
/// See [`ErrorResponses`](https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html)
#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
#[non_exhaustive]
pub enum S3ErrorCode {
    /// The specified bucket is not valid.
    InvalidBucketName,
}

impl S3ErrorCode {
    fn status_code(&self) -> StatusCode {
        match self {
            S3ErrorCode::InvalidBucketName => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
