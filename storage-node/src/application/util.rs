use axum::http::HeaderMap;
use std::collections::HashMap;

pub fn parse_request_headers(headers: HeaderMap) -> HashMap<String, String> {
    headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
        .collect()
}
