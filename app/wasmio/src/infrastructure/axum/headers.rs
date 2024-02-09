use axum::http::request::Parts;
use axum::http::HeaderMap;
use axum::{async_trait, extract::FromRequestParts};
use std::convert::Infallible;

#[derive(Debug, Clone)]
pub struct Headers(pub HeaderMap);

#[async_trait]
impl<S> FromRequestParts<S> for Headers
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(Headers(parts.headers.clone()))
    }
}
