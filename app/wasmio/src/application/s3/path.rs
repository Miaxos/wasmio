//! Part of it was taken from `s3-server` crate.
use std::net::IpAddr;

use axum::async_trait;
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use ulid::Ulid;

use super::errors::{S3Error, S3ErrorCodeKind, S3HTTPError};

#[derive(Debug)]
pub enum S3Path {
    Root,
    Bucket { bucket: String },
    Object { bucket: String, key: String },
}

impl S3Path {
    pub fn from_part(
        request_id: &Ulid,
        parts: &Parts,
    ) -> Result<Self, S3HTTPError> {
        let path = parts.uri.path();
        let path = urlencoding::decode(path).map_err(|e| {
            S3HTTPError::custom(
                "",
                request_id.to_string(),
                S3ErrorCodeKind::InvalidURI,
            )
        })?;
        let path = Self::try_from_path(&path).map_err(|e| {
            S3HTTPError::custom(&path, request_id.to_string(), e)
        })?;

        Ok(path)
    }

    /// See [bucket nameing rules](https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html#bucketnamingrules)
    #[must_use]
    pub fn check_bucket_name(name: &str) -> bool {
        if !(3_usize..64).contains(&name.len()) {
            return false;
        }

        if !name.as_bytes().iter().all(|&b| {
            b.is_ascii_lowercase()
                || b.is_ascii_digit()
                || b == b'.'
                || b == b'-'
        }) {
            return false;
        }

        if name
            .as_bytes()
            .first()
            .map(|&b| b.is_ascii_lowercase() || b.is_ascii_digit())
            != Some(true)
        {
            return false;
        }

        if name
            .as_bytes()
            .last()
            .map(|&b| b.is_ascii_lowercase() || b.is_ascii_digit())
            != Some(true)
        {
            return false;
        }

        if name.parse::<IpAddr>().is_ok() {
            return false;
        }

        if name.starts_with("xn--") {
            return false;
        }

        true
    }

    /// The name for a key is a sequence of Unicode characters whose UTF-8
    /// encoding is at most 1,024 bytes long. See [object keys](https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html#object-keys)
    #[must_use]
    pub const fn check_key(key: &str) -> bool {
        key.len() <= 1024
    }

    /// Parse a path-style request
    /// # Errors
    /// Returns an `Err` if the s3 path is invalid
    pub fn try_from_path(path: &str) -> Result<Self, S3Error> {
        let path = if let Some(("", x)) = path.split_once('/') {
            x
        } else {
            return Err(S3ErrorCodeKind::InvalidURI.into());
        };

        if path.is_empty() {
            return Ok(S3Path::Root);
        }

        let (bucket, key) = match path.split_once('/') {
            None => (path, None),
            Some((x, "")) => (x, None),
            Some((bucket, key)) => (bucket, Some(key)),
        };

        if !Self::check_bucket_name(bucket) {
            return Err(S3ErrorCodeKind::InvalidBucketName.into());
        }

        let key = match key {
            None => {
                return Ok(S3Path::Bucket {
                    bucket: bucket.to_string(),
                })
            }
            Some(k) => k,
        };

        if !Self::check_key(key) {
            return Err(S3ErrorCodeKind::KeyTooLongError.into());
        }

        Ok(Self::Object {
            bucket: bucket.to_string(),
            key: key.to_string(),
        })
    }

    #[must_use]
    pub const fn is_root(&self) -> bool {
        matches!(*self, Self::Root)
    }

    #[must_use]
    pub const fn is_bucket(&self) -> bool {
        matches!(*self, Self::Bucket { .. })
    }

    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(*self, Self::Object { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_s3_path_root() {
        insta::assert_debug_snapshot!(S3Path::try_from_path("/"));
    }

    #[test]
    fn parse_s3_path_bucket() {
        insta::assert_debug_snapshot!(S3Path::try_from_path("/bucket"));
    }

    #[test]
    fn parse_s3_path_bucket_2() {
        insta::assert_debug_snapshot!(S3Path::try_from_path("/bucket/"));
    }

    #[test]
    fn parse_s3_path_object() {
        insta::assert_debug_snapshot!(S3Path::try_from_path(
            "/bucket/dir/object"
        ));
    }

    #[test]
    fn parse_s3_path_fail() {
        insta::assert_debug_snapshot!(S3Path::try_from_path("asd"));
    }

    #[test]
    fn parse_s3_path_fail_2() {
        insta::assert_debug_snapshot!(S3Path::try_from_path("a/"));
    }

    #[test]
    fn parse_s3_path_fail_3() {
        insta::assert_debug_snapshot!(S3Path::try_from_path("/*"));
    }

    #[test]
    fn parse_s3_path_fail_4() {
        let too_long_path = format!("/{}/{}", "asd", "b".repeat(2048).as_str());
        insta::assert_debug_snapshot!(S3Path::try_from_path(&too_long_path));
    }
}
