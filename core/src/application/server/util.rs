use axum::http::{HeaderMap, StatusCode};

pub fn to_tokio_async_read(r: impl futures::io::AsyncRead) -> impl tokio::io::AsyncRead {
    tokio_util::compat::FuturesAsyncReadCompatExt::compat(r)
}

pub fn map_axum_to_std_io_err(err: axum::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

pub fn get_content_length(headers: HeaderMap) -> Result<usize, (StatusCode, String)> {
    let content_length = match headers.get("content-length") {
        Some(header_val) => match header_val.to_str() {
            Ok(header_val) => match header_val.parse::<usize>() {
                Ok(content_length) => content_length,
                Err(e) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        format!("error parsing header content-length: {}", e.to_string()),
                    ))
                }
            },
            Err(e) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("error parsing header content-length: {}", e.to_string()),
                ))
            }
        },
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("missing content-length header"),
            ))
        }
    };
    Ok(content_length)
}
