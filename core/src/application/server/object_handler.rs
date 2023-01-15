use axum::extract::{Path, State};
use axum::http::{HeaderMap, Method, StatusCode};

use std::sync::Arc;
use tracing::instrument;

use crate::domain::object::Object;
use crate::domain::object_service::ObjectService;
use hyper::body::Bytes;

pub struct APIObjectHandler {
    pub object_service: Arc<ObjectService>,
}

#[allow(unused)]
#[instrument(skip(handler, headers, bytes), fields(req.body.len = bytes.len()))]
pub async fn put_object(
    State(handler): State<Arc<APIObjectHandler>>,
    Path(key): Path<String>,
    method: Method,
    headers: HeaderMap,
    bytes: Bytes,
) -> Result<(), (StatusCode, String)> {
    let object = Object::new(key, bytes);
    handler.object_service.put_object(object).await;
    Ok(())
}
