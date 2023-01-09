use axum::extract::{Path as RequestPath, State};
use axum::http::{HeaderMap, Method, StatusCode};

use std::sync::Arc;

use tracing::{info, instrument};

use crate::application::util::parse_request_headers;
use crate::domain::object_service::ObjectService;
use hyper::body::Bytes;

pub struct APIObjectHandler {
    pub object_service: Arc<ObjectService>,
}

#[instrument(skip(handler))]
pub async fn put(
    State(handler): State<Arc<APIObjectHandler>>,
    RequestPath(key): RequestPath<String>,
    method: Method,
    headers: HeaderMap,
    bytes: Bytes,
) -> Result<(), (StatusCode, String)> {
    let parsed_req_headers = parse_request_headers(headers);
    info!(
        req.method = %method,
        req.headers = ?parsed_req_headers,
        "parsed request headers",
    );
    // if let Err(msg) = object_key_is_valid(key.as_str()) {
    //     return Err((StatusCode::BAD_REQUEST, msg));
    // };
    //
    // let key_path = match object_key_is_valid(key.as_str()) {
    //     Ok(path) => path,
    //     Err(msg) => return Err((StatusCode::BAD_REQUEST, msg)),
    // };

    info!(req.key = key, "parsed object key");

    handler.object_service.put(bytes).await;

    Ok(())
}
