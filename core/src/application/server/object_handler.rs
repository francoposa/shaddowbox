use crate::application::server::util;
use crate::domain::object::ObjectStream;
use crate::domain::object_service::ObjectService;
use axum::extract::{BodyStream, Path, State};
use axum::http::{HeaderMap, Method, StatusCode};
use futures_util::TryStreamExt;
use std::fmt::format;
use std::sync::Arc;
use tracing::instrument;

pub struct APIObjectHandler {
    pub object_service: Arc<ObjectService>,
}

#[allow(unused)]
#[axum::debug_handler]
#[instrument(skip(handler, body_stream))]
pub async fn put_object(
    State(handler): State<Arc<APIObjectHandler>>,
    Path(key): Path<String>,
    method: Method,
    headers: HeaderMap,
    mut body_stream: BodyStream,
) -> Result<(), (StatusCode, String)> {
    let body_stream_with_io_error = body_stream.map_err(|err| util::map_axum_to_std_io_err(err));
    let mut stream_reader = util::to_tokio_async_read(body_stream_with_io_error.into_async_read());

    let content_length = util::get_content_length(headers)?;

    let mut object = ObjectStream::new(key, Box::new(stream_reader), content_length);
    handler.object_service.put_object(object).await;
    Ok(())
}
