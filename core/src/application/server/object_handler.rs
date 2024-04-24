use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Method, StatusCode};
use futures_util::TryStreamExt;
use tracing::instrument;

use crate::application::server::util;
use crate::domain::object::ObjectStream;
use crate::domain::object_service::ObjectService;

pub struct APIObjectHandler {
    pub object_service: Arc<ObjectService>,
}

#[allow(unused)]
#[axum::debug_handler]
#[instrument(skip(handler, body))]
pub async fn put_object(
    State(handler): State<Arc<APIObjectHandler>>,
    Path(key): Path<String>,
    method: Method,
    headers: HeaderMap,
    body: Body,
) -> Result<(), (StatusCode, String)> {
    let content_length = util::get_content_length(headers)?;

    let body_stream_with_io_error = body
        .into_data_stream()
        .map_err(|err| util::map_axum_to_std_io_err(err));

    let stream_reader = tokio_util::io::StreamReader::new(body_stream_with_io_error);
    let mut stream_reader = Box::new(Box::pin(stream_reader));

    let mut object = ObjectStream::new(key, stream_reader, content_length);

    handler.object_service.put_object(object).await;
    Ok(())
}
