use bytes::Bytes;
use tokio::io::AsyncRead;

type ObjectStreamReader = Box<dyn AsyncRead + Send + Sync + Unpin>;

pub struct ObjectStream {
    pub key: String,
    pub stream_reader: ObjectStreamReader,
    pub len_bytes: usize,
}
impl ObjectStream {
    pub fn new(key: String, stream_reader: ObjectStreamReader, len_bytes: usize) -> Self {
        ObjectStream {
            key,
            stream_reader,
            len_bytes,
        }
    }
}

pub struct ObjectStripe {
    pub key: String,
    pub bytes: Bytes,
}

impl ObjectStripe {
    pub fn new(key: String, bytes: Bytes) -> Self {
        ObjectStripe { key, bytes }
    }
}
