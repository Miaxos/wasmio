use axum::body::{Body, BodyDataStream};
use axum::extract::{Request, State};
use axum::http::header::{
    CACHE_CONTROL, CONTENT_DISPOSITION, CONTENT_ENCODING, CONTENT_LANGUAGE,
    CONTENT_LENGTH, CONTENT_TYPE, ETAG,
};
use axum::http::StatusCode;
use axum::response::Response;
use tracing::{info, warn};
use wasmio_aws_types::types::PutObjectRequestBuilder;

use crate::application::s3::axum::request_context::RequestContext;
use crate::application::s3::axum::{header_parse, header_string_opt};
use crate::application::s3::errors::{S3ErrorCodeKind, S3HTTPError};
use crate::application::s3::headers::{self, X_AMZ_STORAGE_CLASS};
use crate::application::s3::state::S3State;
use crate::domain::storage::BackendDriver;
use crate::infrastructure::axum::headers::Headers;

pub async fn object_put_handle<T: BackendDriver>(
    req: RequestContext,
    Headers(map): Headers,
    State(state): State<S3State<T>>,
    request: Request,
) -> Result<Response, S3HTTPError> {
    info!(
        message = "Trying to insert a new element",
        bucket = req.bucket()
    );

    let key = match req.object() {
        // TODO: Put the proper error
        None => return Err(req.from_error_code(S3ErrorCodeKind::InternalError)),
        Some(key) => key.to_string(),
    };

    // switch to https://docs.rs/http-body-util/latest/http_body_util/struct.BodyStream.html
    // to have trailers which will be needed for the full implementation
    let stream: BodyDataStream = request.into_body().into_data_stream();

    let request = PutObjectRequestBuilder::default()
        .bucket(req.bucket())
        .body(Some(stream))
        .cache_control(header_string_opt(CACHE_CONTROL, &map))
        .content_type(header_string_opt(CONTENT_TYPE, &map))
        .content_language(header_string_opt(CONTENT_LANGUAGE, &map))
        .content_encoding(header_string_opt(CONTENT_ENCODING, &map))
        .content_disposition(header_string_opt(CONTENT_DISPOSITION, &map))
        .key(key)
        .acl(header_string_opt(headers::X_AMZ_ACL, &map))
        .storage_class(header_string_opt(X_AMZ_STORAGE_CLASS, &map))
        .content_length(header_parse(CONTENT_LENGTH, &map))
        .build();

    // TODO: Other fields too.

    if let Err(err) = request {
        warn!("{err:?}");
        return Err(req.from_error_code(S3ErrorCodeKind::InternalError));
    }

    let insert_task =
        state.bucket_loader.put_object(request.expect("can't fail"));
    insert_task.await.map_err(|x| req.from_error_code(x))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(ETAG, "unimplemented")
        /*
        .header(headers::X_AMZ_EXPIRATION, unimplemented!(""))
        .header(headers::X_AMZ_CONTENT_SHA_256, unimplemented!(""))
        .header(headers::X_AMZ_SERVER_SIDE_ENCRYPTION, unimplemented!(""))
        .header(headers::X_AMZ_VERSION_ID, unimplemented!(""))
        .header(
            headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM,
            unimplemented!(""),
        )
        .header(
            headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5,
            unimplemented!(""),
        )
        .header(
            headers::X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID,
            unimplemented!(""),
        )
        .header(
            headers::X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT,
            unimplemented!(""),
        )
        .header(
            headers::X_AMZ_BUCKET_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED,
            unimplemented!(""),
        )
                */
        .body(Body::empty())
        .unwrap())
}
