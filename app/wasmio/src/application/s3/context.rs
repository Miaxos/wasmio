use axum::async_trait;
use axum::body::Body;
use axum::http::Request;
use axum::response::Response;

use super::errors::{S3Error, S3ErrorCodeKind, S3HTTPError};
use super::state::S3State;
use crate::domain::storage::BackendDriver;

/// Request Context
#[derive(Debug)]
pub struct Context {
    pub request: Request<Body>,
}

impl Context {
    pub fn new(request: Request<Body>) -> Self {
        Self { request }
    }

    pub fn bucket_name(&self) -> Option<String> {
        todo!()
    }

    pub fn expect_bucket(&self) -> Result<String, S3Error> {
        Err(S3ErrorCodeKind::InvalidBucketName.into())
    }
}

/// The routing of S3 is based on URI matching, it conflicts with the usual way
/// we do routing so we use this trait to be implemented on each
///
/// (Idea taken from s3-server.)
#[async_trait]
pub trait S3Handler {
    /// Function to implement on each handler to confirm the handler should be
    /// able to answer this request.
    fn is_match(&self, ctx: &Context) -> bool;

    async fn handle<T: BackendDriver>(
        &self,
        ctx: Context,
        state: S3State<T>,
    ) -> Result<Response, S3Error>;
}

#[derive(Clone, Copy)]
pub struct VisitorNil;

#[async_trait]
impl S3Handler for VisitorNil {
    #[inline]
    fn is_match(&self, ctx: &Context) -> bool {
        false
    }

    async fn handle<T: BackendDriver>(
        &self,
        ctx: Context,
        state: S3State<T>,
    ) -> Result<Response, S3Error> {
        unreachable!("shouldn't be called.")
    }
}

impl VisitorNil {
    pub(crate) const fn with<V>(self, visitor: V) -> VisitorCons<V, Self> {
        VisitorCons(visitor, self)
    }
}

pub struct VisitorCons<A, B>(A, B);

impl<A, B> VisitorCons<A, B> {
    pub(crate) const fn with<V>(self, visitor: V) -> VisitorCons<V, Self> {
        VisitorCons(visitor, self)
    }
}

impl<A: Clone, B: Clone> Clone for VisitorCons<A, B> {
    fn clone(&self) -> Self {
        VisitorCons(self.0.clone(), self.1.clone())
    }
}

impl<A: Copy, B: Copy> Copy for VisitorCons<A, B> {}

#[async_trait]
impl<A: S3Handler + Send + Sync, B: S3Handler + Send + Sync> S3Handler
    for VisitorCons<A, B>
{
    #[inline]
    fn is_match(&self, ctx: &Context) -> bool {
        self.0.is_match(ctx) || self.0.is_match(ctx)
    }

    async fn handle<T: BackendDriver>(
        &self,
        ctx: Context,
        state: S3State<T>,
    ) -> Result<Response, S3Error> {
        if self.0.is_match(&ctx) {
            return self.0.handle(ctx, state).await;
        }

        self.1.handle(ctx, state).await
    }
}
