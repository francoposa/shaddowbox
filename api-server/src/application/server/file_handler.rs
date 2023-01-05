use axum::http::{HeaderMap, Method};
use tracing::{info, instrument};

use crate::application::util::parse_request_headers;
use hyper::body::Bytes;

#[instrument]
pub async fn put(method: Method, headers: HeaderMap, bytes: Bytes) -> Bytes {
    let parsed_req_headers = parse_request_headers(headers);
    info!(
        req.method = %method,
        req.headers = ?parsed_req_headers,
        "parsed request headers",
    );
    bytes
}
