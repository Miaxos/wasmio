use axum::async_trait;
use axum::body::{Body, BodyDataStream};
use axum::extract::{FromRequestParts, Query, Request, State};
use axum::http::header::{
    self, CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_ENCODING,
    CONTENT_LANGUAGE, CONTENT_LENGTH, CONTENT_TYPE, ETAG,
};
use axum::http::{Method, StatusCode};
use axum::response::Response;
use if_chain::if_chain;
use tracing::{error, info, warn};
use wasmio_aws_types::types::{
    ListObjectsV2Output, ListObjectsV2Request, PutObjectRequestBuilder,
};

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
pub struct ObjectListHandlerV2;

#[derive(serde::Deserialize)]
pub struct ObjectListV2QS {
    #[serde(rename = "list-type")]
    list_type: i8,
    #[serde(rename = "continuation-token")]
    continuation_token: Option<String>,
    delimiter: Option<String>,
    #[serde(rename = "encoding-type")]
    encoding_type: Option<String>,
    #[serde(rename = "fetch-owner")]
    fetch_owner: Option<bool>,
    #[serde(rename = "max-keys")]
    max_keys: Option<i64>,
    #[serde(rename = "prefix")]
    prefix: Option<String>,
    #[serde(rename = "start-after")]
    start_after: Option<String>,
}

#[async_trait]
impl S3Handler for ObjectListHandlerV2 {
    #[inline]
    fn is_match(&self, ctx: &Context) -> bool {
        if_chain! {
            if ctx.method() == Method::GET;
            if ctx.path().is_bucket();
            if let Ok(Query(qs)) = Query::<ObjectListV2QS>::try_from_uri(&ctx.parts().uri);
            if qs.list_type == 2;
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
        let bucket_name = ctx.expect_bucket()?;
        let Query(ObjectListV2QS {
            list_type: _,
            continuation_token,
            delimiter,
            encoding_type,
            fetch_owner,
            max_keys,
            prefix,
            start_after,
        }) = Query::<ObjectListV2QS>::try_from_uri(&ctx.parts().uri)
            .expect("Can't fail as we already checked.");

        info!(
            message = "Trying to list elements",
            bucket = %bucket_name,
        );
        let map = &ctx.parts().headers;

        let request = ListObjectsV2Request {
            bucket: bucket_name.into(),
            encoding_type,
            fetch_owner,
            max_keys,
            prefix,
            start_after,
            continuation_token,
            delimiter,
            expected_bucket_owner: header_string_opt(
                headers::X_AMZ_EXPECTED_BUCKET_OWNER,
                map,
            ),
            request_payer: header_string_opt(headers::X_AMZ_REQUEST_PAYER, map),
        };

        let result = state.bucket_loader.list_object_v2(request).await?;

        let xml = quick_xml::se::to_string(&result).map_err(|err| {
            warn!("{err}");
            S3Error::from(S3ErrorCodeKind::MalformedXML)
        })?;

        let body = format!(
            r###"<?xml version="1.0" encoding="UTF-8"?>
{xml}
"###,
            xml = xml
        );

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header_opt(headers::X_AMZ_REQUEST_CHARGED, Some("unimplemented"))
            .body(Body::new(body))
            .unwrap())
    }
}
