use axum::extract::{BodyStream, Path as RequestPath, State};
use axum::http::{HeaderMap, Method, StatusCode};
use std::path::Path;
use std::sync::Arc;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};
use tracing::{info, instrument};

use crate::application::util::parse_request_headers;
use hyper::body::Bytes;

#[derive(Debug)]
pub struct APIObjectHandler {
    pub file_dir: String,
}

#[instrument]
pub async fn put(
    State(handler): State<Arc<APIObjectHandler>>,
    RequestPath(key): RequestPath<String>,
    method: Method,
    headers: HeaderMap,
    bytes: Bytes,
) -> Result<Bytes, (StatusCode, String)> {
    let parsed_req_headers = parse_request_headers(headers);
    info!(
        req.method = %method,
        req.headers = ?parsed_req_headers,
        "parsed request headers",
    );
    if let Err(msg) = object_key_is_valid(key.as_str()) {
        return Err((StatusCode::BAD_REQUEST, msg));
    };

    let key_path = match object_key_is_valid(key.as_str()) {
        Ok(path) => path,
        Err(msg) => return Err((StatusCode::BAD_REQUEST, msg)),
    };

    info!(req.key = key, "parsed object key");
    Ok(bytes)
}

// validate path contains all normal components to prevent directory traversal attacks
fn object_key_is_valid(object_key: &str) -> Result<&Path, String> {
    let filepath = Path::new(object_key);
    for component in filepath.components().into_iter() {
        if !matches!(component, std::path::Component::Normal(_)) {
            return Err(String::from("invalid object key"));
        }
    }
    Ok(filepath)
}
