use axum::async_trait;
use axum::body::Body;
use axum::http::request::Parts;
use axum::http::{Method, Request, StatusCode};
use axum::response::Response;
use tracing::error;
use ulid::Ulid;

use super::errors::{S3Error, S3ErrorCodeKind, S3HTTPError};
use super::path::S3Path;
use super::state::S3State;
use crate::domain::storage::BackendDriver;

/// Request Context
#[derive(Debug)]
pub struct Context {
    pub request_id: Ulid,
    pub parts: Parts,
    pub body: Body,
    path: S3Path,
}

impl Context {
    pub fn new(request: Request<Body>) -> Result<Self, S3HTTPError> {
        let request_id = Ulid::new();

        let (parts, body) = request.into_parts();

        let path = S3Path::from_part(&request_id, &parts)?;

        Ok(Self {
            request_id,
            parts,
            body,
            path,
        })
    }

    pub fn path(&self) -> &S3Path {
        &self.path
    }

    pub fn expect_bucket(&self) -> Result<&String, S3Error> {
        match &self.path {
            S3Path::Bucket { bucket } => Ok(bucket),
            _ => {
                error!("You should have a bucket here, weird, investigate.");
                Err(S3Error::invalid_request("You should have a Bucket here."))
            }
        }
    }

    pub fn expect_object(&self) -> Result<(&String, &String), S3Error> {
        match &self.path {
            S3Path::Object { bucket, key } => Ok((bucket, key)),
            _ => {
                error!("You should have an object here, weird, investigate.");
                Err(S3Error::invalid_request("You should have an Object here."))
            }
        }
    }

    pub fn expect_root(&self) -> Result<(), S3Error> {
        match &self.path {
            S3Path::Root => Ok(()),
            _ => {
                error!("You should have root here, weird, investigate.");
                Err(S3Error::invalid_request("You should have a Root here."))
            }
        }
    }

    pub fn body(&mut self) -> Body {
        std::mem::take(&mut self.body)
    }

    pub fn request_id(&self) -> Ulid {
        self.request_id
    }

    pub fn resource(&self) -> String {
        match &self.path {
            S3Path::Root => "/".to_string(),
            S3Path::Bucket { bucket } => format!("/{bucket}/"),
            S3Path::Object { bucket, key } => format!("/{bucket}/{key}"),
        }
    }

    pub fn method(&self) -> &Method {
        &self.parts.method
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
        _ctx: Context,
        _state: S3State<T>,
    ) -> Result<Response, S3Error> {
        Err(S3Error::invalid_request("This pattern has not been implemented yet, feel free to drop an issue at `https://github.com/miaxos/wasmio`. 💜"))
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
            self.0.handle(ctx, state).await
        } else {
            self.1.handle(ctx, state).await
        }
    }
}
