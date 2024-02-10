use axum::async_trait;
use axum::extract::rejection::{HostRejection, PathRejection};
use axum::extract::{FromRequestParts, Host};
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum_extra::extract::OptionalPath;
use ulid::Ulid;

use crate::application::s3::errors::{S3ErrorCodeKind, S3HTTPError};

#[derive(Clone, Debug)]
pub struct RequestContext {
    request_id: Ulid,
    bucket: String,
    object: Option<String>,
}

impl RequestContext {
    #[allow(dead_code)]
    pub fn request_id(&self) -> &Ulid {
        &self.request_id
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn object(&self) -> Option<&str> {
        self.object.as_deref()
    }

    pub fn ressource(&self) -> String {
        let obj = self.object().unwrap_or_default();
        format!("{bucket}/{obj}", bucket = self.bucket(), obj = obj)
    }

    pub fn to_error_code(
        &self,
        code: impl Into<S3ErrorCodeKind>,
    ) -> S3HTTPError {
        S3HTTPError::custom(
            self.ressource(),
            self.request_id.to_string(),
            code.into(),
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RequestContextRejection {
    #[error("Host: {0}")]
    Host(#[from] HostRejection),
    #[error("Path: {0}")]
    Path(#[from] PathRejection),
    #[error("Other: {0:?}")]
    Other(#[from] S3HTTPError),
}

impl IntoResponse for RequestContextRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            RequestContextRejection::Host(host) => host.into_response(),
            RequestContextRejection::Path(path) => path.into_response(),
            RequestContextRejection::Other(other) => other.into_response(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RequestContext
where
    S: Send + Sync,
{
    type Rejection = RequestContextRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let path =
            OptionalPath::<String>::from_request_parts(parts, state).await?;
        let host = Host::from_request_parts(parts, state).await?;

        let request_id = Ulid::new();

        let bucket = match host.0.split('.').next() {
            Some(elt) => elt.to_string(),
            None => {
                return Err(RequestContextRejection::Other(
                    S3HTTPError::custom(
                        "",
                        request_id.to_string(),
                        S3ErrorCodeKind::InvalidBucketName,
                    ),
                ));
            }
        };

        Ok(RequestContext {
            request_id,
            bucket,
            object: path.0,
        })
    }
}
