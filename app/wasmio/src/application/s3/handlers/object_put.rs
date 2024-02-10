use axum::async_trait;
use axum::body::{Body, BodyDataStream};
use axum::extract::{Request, State};
use axum::http::header::{
    self, CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_ENCODING,
    CONTENT_LANGUAGE, CONTENT_LENGTH, CONTENT_TYPE, ETAG,
};
use axum::http::{Method, StatusCode};
use axum::response::Response;
use if_chain::if_chain;
use tracing::{error, info, warn};
use wasmio_aws_types::types::PutObjectRequestBuilder;

use crate::application::s3::axum::request_context::RequestContext;
use crate::application::s3::axum::{
    header_parse, header_string_opt, RequestExt,
};
use crate::application::s3::context::{Context, S3Handler};
use crate::application::s3::errors::{S3Error, S3ErrorCodeKind, S3HTTPError};
use crate::application::s3::headers::{self, X_AMZ_STORAGE_CLASS};
use crate::application::s3::state::S3State;
use crate::domain::storage::errors::BucketStorageError;
use crate::domain::storage::BackendDriver;
use crate::infrastructure::axum::headers::Headers;
use crate::infrastructure::storage::BackendStorage;

#[derive(Clone, Copy)]
pub struct ObjectPutHandler;

#[async_trait]
impl S3Handler for ObjectPutHandler {
    #[inline]
    fn is_match(&self, ctx: &Context) -> bool {
        // Only support normal put for now, multipart later
        if_chain! {
            if ctx.method() == Method::PUT;
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
        mut ctx: Context,
        state: S3State<T>,
    ) -> Result<Response, S3Error>
    where
        BucketStorageError: From<<T as BackendStorage>::Error>,
    {
        let body = ctx.body();
        let (bucket_name, key) = ctx.expect_object()?;

        info!(
            message = "Trying to insert a new element",
            bucket = %bucket_name,
            key = %key,
        );

        // switch to https://docs.rs/http-body-util/latest/http_body_util/struct.BodyStream.html
        // to have trailers which will be needed for the full implementation
        let stream: BodyDataStream = body.into_data_stream();
        let map = &ctx.parts.headers;

        let request = PutObjectRequestBuilder::default()
            .bucket(bucket_name)
            .body(Some(stream))
            .content_length(header_parse(CONTENT_LENGTH, &map).map_err(
                |_err| {
                    S3Error::invalid_request("Invalid header: content-length")
                },
            )?)
            .acl(header_string_opt(headers::X_AMZ_ACL, &map))
            .cache_control(header_string_opt(CACHE_CONTROL, &map))
            .content_type(header_string_opt(CONTENT_TYPE, &map))
            .content_language(header_string_opt(CONTENT_LANGUAGE, &map))
            .content_encoding(header_string_opt(CONTENT_ENCODING, &map))
            .content_disposition(header_string_opt(CONTENT_DISPOSITION, &map))
            .storage_class(header_string_opt(X_AMZ_STORAGE_CLASS, &map))
            .key(key)
            .build();

        if let Err(err) = request {
            error!("{err:?}");
            return Err(S3Error::invalid_request(
                "Server error, please check repo or contact admin.",
            ));
        }

        let insert_task =
            state.bucket_loader.put_object(request.expect("can't fail"));
        let output = insert_task.await?;

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header_opt(ETAG, output.e_tag)
            .header_opt(headers::X_AMZ_EXPIRATION, output.expiration)
            .header_opt(headers::X_AMZ_CONTENT_SHA_256, output.checksum)
            .header_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION,
                output.server_side_encryption,
            )
            .header_opt(headers::X_AMZ_VERSION_ID, output.version_id)
            .header_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
                output.sse_customer_algorithm,
            )
            .header_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
                output.sse_customer_key_md5,
            )
            .header_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID,
                output.ssekms_key_id,
            )
            .header_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT,
                output.ssekms_encryption_context,
            )
            .header_opt(
                headers::X_AMZ_BUCKET_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
                output.bucket_key_enabled.map(|x| match x {
                    true => "true",
                    false => "false",
                }),
            )
            .body(Body::empty())
            .unwrap())
    }
}
