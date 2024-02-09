use std::str::FromStr;
use tracing::warn;

use crate::{
    application::s3::{
        axum::{
            header_parse, header_parse_bool, header_string_opt, request_context::RequestContext,
        },
        errors::S3ErrorCodeKind,
        headers::{self, X_AMZ_STORAGE_CLASS},
    },
    domain::storage::BackendDriver,
    infrastructure::axum::headers::Headers,
};
use axum::{
    body::{Body, BodyDataStream},
    extract::{Request, State},
    http::{
        header::{
            AsHeaderName, CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_ENCODING, CONTENT_LANGUAGE,
            CONTENT_LENGTH, CONTENT_TYPE, ETAG,
        },
        HeaderMap, StatusCode,
    },
    response::Response,
};
use serde_aws_types::types::{DeleteObjectRequestBuilder, PutObjectRequestBuilder};
use tracing::info;

use crate::application::s3::{errors::S3HTTPError, state::S3State};

pub async fn object_delete_handle<T: BackendDriver>(
    // TODO: Add version id from req
    req: RequestContext,
    Headers(map): Headers,
    State(state): State<S3State<T>>,
    request: Request,
) -> Result<Response, S3HTTPError> {
    info!(
        message = "Trying to delete an element",
        bucket = req.bucket()
    );

    let key = match req.object() {
        // TODO: Put the proper error
        None => return Err(req.from_error_code(S3ErrorCodeKind::InternalError)),
        Some(key) => key.to_string(),
    };

    let request = DeleteObjectRequestBuilder::default()
        .bucket(req.bucket())
        .key(key)
        .mfa(header_string_opt(headers::X_AMZ_MFA, &map))
        .request_payer(header_string_opt(headers::X_AMZ_REQUEST_PAYER, &map))
        .bypass_governance_retention(header_parse_bool(
            headers::X_AMZ_BYPASS_GOVERNANCE_RETENTION,
            &map,
        ))
        .expected_bucket_owner(header_string_opt(
            headers::X_AMZ_EXPECTED_BUCKET_OWNER,
            &map,
        ))
        .build();

    if let Err(err) = request {
        warn!("{err:?}");
        return Err(req.from_error_code(S3ErrorCodeKind::InternalError));
    }

    let insert_task = state
        .bucket_loader
        .delete_object(request.expect("can't fail"));
    insert_task.await.map_err(|x| req.from_error_code(x))?;

    todo!()
    // todo
}
