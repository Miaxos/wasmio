use axum::async_trait;
use axum::body::Body;
use axum::http::{Method, StatusCode};
use axum::response::Response;
use if_chain::if_chain;
use tracing::{error, info};
use wasmio_aws_types::types::DeleteObjectRequestBuilder;

use crate::application::s3::axum::{
    header_parse_bool, header_string_opt, RequestExt,
};
use crate::application::s3::context::{Context, S3Handler};
use crate::application::s3::errors::S3Error;
use crate::application::s3::headers::{self};
use crate::application::s3::state::S3State;
use crate::domain::storage::errors::BucketStorageError;
use crate::domain::storage::BackendDriver;
use crate::infrastructure::storage::BackendStorage;

#[derive(Clone, Copy)]
pub struct ObjectDeleteHandler;

#[async_trait]
impl S3Handler for ObjectDeleteHandler {
    #[inline]
    fn is_match(&self, ctx: &Context) -> bool {
        if_chain! {
            if ctx.method() == Method::DELETE;
            if ctx.path().is_object();
            then {
                true
            } else {
                false
            }
        }
    }

    async fn handle<T: BackendDriver>(
        &self,
        ctx: Context,
        state: S3State<T>,
    ) -> Result<Response, S3Error>
    where
        BucketStorageError: From<<T as BackendStorage>::Error>,
    {
        let (bucket_name, key) = ctx.expect_object()?;

        info!(
            message = "Trying to delete an element",
            bucket = %bucket_name,
            key = %key,
        );

        let map = &ctx.parts().headers;

        let request = DeleteObjectRequestBuilder::default()
            .bucket(bucket_name)
            .key(key)
            .mfa(header_string_opt(headers::X_AMZ_MFA, map))
            .request_payer(header_string_opt(headers::X_AMZ_REQUEST_PAYER, map))
            .bypass_governance_retention(header_parse_bool(
                headers::X_AMZ_BYPASS_GOVERNANCE_RETENTION,
                map,
            ))
            .expected_bucket_owner(header_string_opt(
                headers::X_AMZ_EXPECTED_BUCKET_OWNER,
                map,
            ))
            .build();

        if let Err(err) = request {
            error!("{err:?}");
            return Err(S3Error::invalid_request(
                "Server error, please check repo or contact admin.",
            ));
        }

        let insert_task = state
            .bucket_loader
            .delete_object(request.expect("can't fail"));
        let output = insert_task.await?;

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header_opt(headers::X_AMZ_VERSION_ID, output.version_id)
            .header_opt(
                headers::X_AMZ_DELETE_MARKER,
                output.delete_marker.map(|x| match x {
                    true => "true",
                    false => "false",
                }),
            )
            .header_opt(headers::X_AMZ_REQUEST_CHARGED, output.request_charged)
            .body(Body::empty())
            .unwrap())
    }
}
