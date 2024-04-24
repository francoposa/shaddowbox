#[allow(unused)]
pub fn to_tokio_async_read(r: impl futures::io::AsyncRead) -> impl tokio::io::AsyncRead {
    tokio_util::compat::FuturesAsyncReadCompatExt::compat(r)
}

pub fn map_axum_to_std_io_err(err: axum::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

pub fn get_content_length(
    headers: axum::http::HeaderMap,
) -> Result<usize, (axum::http::StatusCode, String)> {
    let content_length = match headers.get("content-length") {
        Some(header_val) => match header_val
            .to_str()
            .map_err(|e| e.to_string())
            .and_then(|header_val| header_val.parse::<usize>().map_err(|e| e.to_string()))
        {
            Ok(content_length) => content_length,
            Err(e) => {
                return Err((
                    axum::http::StatusCode::BAD_REQUEST,
                    format!("error parsing header content-length: {}", e),
                ))
            }
        },
        None => {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                String::from("missing content-length header"),
            ))
        }
    };

    Ok(content_length)
}
