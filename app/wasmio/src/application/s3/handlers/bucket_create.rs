use axum::async_trait;
use axum::body::Body;
use axum::extract::FromRequest;
use axum::http::{header, Method, Request, StatusCode};
use axum::response::Response;
use axum_serde::xml::Xml;
use if_chain::if_chain;
use tracing::{error, info};
use wasmio_aws_types::types::{
    CreateBucketConfiguration, CreateBucketRequestBuilder,
};

use crate::application::s3::axum::RequestExt;
use crate::application::s3::context::{Context, S3Handler};
use crate::application::s3::errors::{S3Error, S3ErrorCodeKind};
use crate::application::s3::state::S3State;
use crate::domain::storage::errors::BucketStorageError;
use crate::domain::storage::BackendDriver;
use crate::infrastructure::storage::BackendStorage;

#[derive(Clone, Copy)]
pub struct BucketCreateHandler;

#[async_trait]
impl S3Handler for BucketCreateHandler {
    #[inline]
    fn is_match(&self, ctx: &Context) -> bool {
        if_chain! {
            if ctx.method() == Method::PUT;
            if ctx.path().is_bucket();
            then {
                true
            } else {
                false
            }
        }
    }

    async fn handle<T: BackendDriver>(
        &self,
        mut ctx: Context,
        state: S3State<T>,
    ) -> Result<Response, S3Error>
    where
        BucketStorageError: From<<T as BackendStorage>::Error>,
    {
        let body = ctx.body();
        let bucket_name = ctx.expect_bucket()?;

        // Useless clone, but it'll do for now;
        let parts = ctx.parts().clone();
        let request = Request::from_parts(parts, body);

        let _input =
            Xml::<CreateBucketConfiguration>::from_request(request, &())
                .await
                .map_err(|_| S3ErrorCodeKind::MalformedXML)?;

        let request = CreateBucketRequestBuilder::default()
            .bucket(bucket_name)
            .build();

        if let Err(err) = request {
            error!("{err:?}");
            return Err(S3Error::invalid_request(
                "Server error, please check repo or contact admin.",
            ));
        }

        info!(message = "Creating a new bucket");
        let result = state
            .bucket_loader
            .create_new_bucket(request.expect("can't fail"))
            .await?;

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header_opt(header::LOCATION, result.location)
            .body(Body::empty())
            .unwrap())
    }
}
