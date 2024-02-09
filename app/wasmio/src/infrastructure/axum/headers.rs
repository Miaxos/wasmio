use std::convert::Infallible;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::HeaderMap;

#[derive(Debug, Clone)]
pub struct Headers(pub HeaderMap);

#[async_trait]
impl<S> FromRequestParts<S> for Headers
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Headers(parts.headers.clone()))
    }
}
