use std::str::FromStr;

use axum::http::{header::AsHeaderName, HeaderMap};

pub mod request_context;

pub fn header_string_opt<K: AsHeaderName>(key: K, map: &HeaderMap) -> Option<String> {
    map.get(key)
        .and_then(|x| x.to_str().map(|x| x.to_string()).ok())
}

pub fn header_parse<K: AsHeaderName, T: FromStr>(key: K, map: &HeaderMap) -> Option<T> {
    header_string_opt(key, map).and_then(|x| x.parse::<T>().ok())
}

pub fn header_parse_bool<K: AsHeaderName>(key: K, map: &HeaderMap) -> Option<bool> {
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
