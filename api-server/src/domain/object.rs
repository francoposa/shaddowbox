use bytes::Bytes;

pub struct Object {
    pub key: String,
    pub bytes: Bytes,
}
impl Object {
    pub fn new(key: String, bytes: Bytes) -> Self {
        Object { key, bytes }
    }
}
