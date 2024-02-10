use std::str::FromStr;

use axum::http::header::AsHeaderName;
use axum::http::response::Builder;
use axum::http::{HeaderMap, HeaderName, HeaderValue};

pub mod request_context;

pub fn header_string_opt<K: AsHeaderName>(
    key: K,
    map: &HeaderMap,
) -> Option<String> {
    map.get(key)
        .and_then(|x| x.to_str().map(|x| x.to_string()).ok())
}

pub fn header_parse<K: AsHeaderName, T: FromStr>(
    key: K,
    map: &HeaderMap,
) -> Result<Option<T>, T::Err> {
    if let Some(s) = header_string_opt(key, map) {
        s.parse::<T>().map(Some)
    } else {
        Ok(None)
    }
}

pub fn header_parse_bool<K: AsHeaderName>(
    key: K,
    map: &HeaderMap,
) -> Option<bool> {
    map.get(key).and_then(|x| {
        x.to_str()
            .map(|x| match x {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            })
            .ok()
            .flatten()
    })
}

pub trait RequestExt {
    /// Appends an optional header to this response builder.
    fn header_opt<K, V>(self, key: K, value: Option<V>) -> Builder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<axum::http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<axum::http::Error>;
}

impl RequestExt for Builder {
    fn header_opt<K, V>(self, key: K, value: Option<V>) -> Builder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<axum::http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<axum::http::Error>,
    {
        if let Some(value) = value {
            self.header(key, value)
        } else {
            self
        }
    }
}
