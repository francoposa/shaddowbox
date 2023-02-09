pub fn to_tokio_async_read(r: impl futures::io::AsyncRead) -> impl tokio::io::AsyncRead {
    tokio_util::compat::FuturesAsyncReadCompatExt::compat(r)
}

pub fn map_axum_to_std_io_err(err: axum::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}
