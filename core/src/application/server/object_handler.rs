use crate::domain::object::ObjectStream;
use crate::domain::object_service::ObjectService;
use axum::extract::{BodyStream, Path, State};
use axum::http;
use axum::http::{HeaderMap, Method, StatusCode};
use futures_util::TryStreamExt;
use std::io;
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
    let body_stream_with_io_error = body_stream.map_err(|err| map_axum_to_std_io_err(err));
    let mut stream_reader = to_tokio_async_read(body_stream_with_io_error.into_async_read());

    let content_length = match headers.get("content-length") {
        Some(header_val) => header_val.to_str().unwrap().parse::<usize>().unwrap(),
        None => {
            return Err((
                http::StatusCode::BAD_REQUEST,
                String::from("missing Content-Length header"),
            ))
        }
    };

    let mut object = ObjectStream::new(key, Box::new(stream_reader), content_length);
    handler.object_service.put_object(object).await;
    Ok(())
}

fn to_tokio_async_read(r: impl futures::io::AsyncRead) -> impl tokio::io::AsyncRead {
    tokio_util::compat::FuturesAsyncReadCompatExt::compat(r)
}

fn map_axum_to_std_io_err(err: axum::Error) -> std::io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}
