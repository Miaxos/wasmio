use axum::async_trait;
use axum::body::{Body, BodyDataStream};
use axum::extract::{FromRequestParts, Query, Request, State};
use axum::http::header::{
    self, CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_ENCODING,
    CONTENT_LANGUAGE, CONTENT_LENGTH, CONTENT_TYPE, ETAG,
};
use axum::http::{HeaderName, HeaderValue, Method, StatusCode};
use axum::response::Response;
use if_chain::if_chain;
use tracing::{error, info, warn};
use wasmio_aws_types::types::{
    GetObjectOutput, GetObjectRequest, ListObjectsV2Output,
    ListObjectsV2Request, PutObjectRequestBuilder,
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
pub struct ObjectAttributesHandler;

#[derive(serde::Deserialize)]
pub struct ObjectAttributesQS {
    #[serde(rename = "attributes")]
    override_cache_control: (),
    #[serde(rename = "versionId")]
    version_id: Option<String>,
}

#[async_trait]
impl S3Handler for ObjectAttributesHandler {
    #[inline]
    fn is_match(&self, ctx: &Context) -> bool {
        if_chain! {
            if ctx.method() == Method::GET;
            if ctx.path().is_object();
            if let Ok(Query(qs)) = Query::<ObjectAttributesQS>::try_from_uri(&ctx.parts().uri);
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
        let Query(qs) =
            Query::<ObjectAttributesQS>::try_from_uri(&ctx.parts().uri)
                .expect("Can't fail as we already checked.");

        info!(
            message = "Trying to get metadata for an element",
            bucket = %bucket_name,
            bucket = %key,
        );
        let map = &ctx.parts().headers;

        let request = GetObjectRequest {
            bucket: bucket_name.to_string(),
            expected_bucket_owner: header_string_opt(
                headers::X_AMZ_EXPECTED_BUCKET_OWNER,
                map,
            ),
            request_payer: header_string_opt(headers::X_AMZ_REQUEST_PAYER, map),
            if_match: header_string_opt(header::IF_MATCH, map),
            if_modified_since: header_string_opt(
                header::IF_MODIFIED_SINCE,
                map,
            ),
            if_none_match: header_string_opt(header::IF_NONE_MATCH, map),
            if_unmodified_since: header_string_opt(
                header::IF_UNMODIFIED_SINCE,
                map,
            ),
            key: key.to_string(),
            part_number: qs.part_number,
            range: header_string_opt(header::RANGE, map),
            response_cache_control: qs.override_cache_control,
            response_content_disposition: qs.override_content_dispositon,
            response_content_encoding: qs.override_content_encoding,
            response_content_language: qs.override_content_language,
            response_content_type: qs.override_content_type,
            response_expires: qs.override_expires,
            sse_customer_algorithm: header_string_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
                map,
            ),
            sse_customer_key: header_string_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY,
                map,
            ),
            sse_customer_key_md5: header_string_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
                map,
            ),
            version_id: qs.version_id,
        };

        let GetObjectOutput {
            accept_ranges,
            body,
            bucket_key_enabled,
            cache_control,
            content_disposition,
            content_encoding,
            content_language,
            content_length,
            content_range,
            content_type,
            delete_marker,
            e_tag,
            expiration,
            expires,
            last_modified,
            metadata,
            missing_meta,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            parts_count,
            replication_status,
            request_charged,
            restore,
            sse_customer_algorithm,
            sse_customer_key_md5,
            ssekms_key_id,
            server_side_encryption,
            storage_class,
            tag_count,
            version_id,
            website_redirect_location,
        } = state.bucket_loader.get_object(request).await?;

        let mut response = Response::builder()
            .status(StatusCode::OK)
            .header_opt(header::ACCEPT_RANGES, accept_ranges)
            .header_opt(header::CACHE_CONTROL, cache_control)
            .header_opt(header::CONTENT_DISPOSITION, content_disposition)
            .header_opt(header::CONTENT_ENCODING, content_encoding)
            .header_opt(header::CONTENT_LANGUAGE, content_language)
            .header_opt(header::CONTENT_LENGTH, content_length)
            .header_opt(header::CONTENT_RANGE, content_range)
            .header_opt(header::CONTENT_TYPE, content_type)
            .header_opt(header::ETAG, e_tag)
            .header_opt(header::EXPIRES, expires)
            .header_opt(header::LAST_MODIFIED, last_modified)
            .header_opt(
                headers::X_AMZ_BUCKET_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
                bucket_key_enabled.map(|x| match x {
                    true => "true",
                    false => "false",
                }),
            )
            .header_opt(headers::X_AMZ_EXPIRATION, expiration)
            .header_opt(
                headers::X_AMZ_DELETE_MARKER,
                delete_marker.map(|x| match x {
                    true => "true",
                    false => "false",
                }),
            )
            .header_opt(headers::X_AMZ_MISSING_META, missing_meta)
            .header_opt(headers::X_AMZ_RESTORE, restore)
            .header_opt(headers::X_AMZ_OBJECT_LOCK_MODE, object_lock_mode)
            .header_opt(
                headers::X_AMZ_OBJECT_LOCK_LEGAL_HOLD,
                object_lock_legal_hold_status,
            )
            .header_opt(
                headers::X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE,
                object_lock_retain_until_date,
            )
            .header_opt(headers::X_AMZ_MP_PARTS_COUNT, parts_count)
            .header_opt(headers::X_AMZ_REPLICATION_STATUS, replication_status)
            .header_opt(headers::X_AMZ_REQUEST_CHARGED, request_charged)
            .header_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
                sse_customer_algorithm,
            )
            .header_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
                sse_customer_key_md5,
            )
            .header_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID,
                ssekms_key_id,
            )
            .header_opt(
                headers::X_AMZ_SERVER_SIDE_ENCRYPTION,
                server_side_encryption,
            )
            .header_opt(headers::X_AMZ_STORAGE_CLASS, storage_class)
            .header_opt(headers::X_AMZ_TAGGING_COUNT, tag_count)
            .header_opt(headers::X_AMZ_VERSION_ID, version_id)
            .header_opt(
                headers::X_AMZ_WEBSITE_REDIRECT_LOCATION,
                website_redirect_location,
            );

        if let Some(metadata) = metadata {
            if let Some(headers) = response.headers_mut() {
                for (key, value) in metadata {
                    let header_name = HeaderName::from_bytes(
                        format!("x-amz-meta-{key}").as_bytes(),
                    );
                    let header_value =
                        HeaderValue::from_bytes(value.as_bytes());

                    if let (Ok(name), Ok(val)) = (header_name, header_value) {
                        headers.insert(name, val);
                    } else {
                        error!(message = "An issue happened with metadata stored in db", key = %key, value = %value);
                    }
                }
            } else {
                warn!("Error while adding custom metadata");
            }
        }

        Ok(response.body(body.unwrap_or_default()).unwrap())
    }
}
