use crate::domain::object_storage_node::ObjectStorageNode;
use bytes::Bytes;
use std::sync::Arc;

pub struct ObjectService {
    storage_node: Arc<dyn ObjectStorageNode + Send + Sync>,
}

impl ObjectService {
    pub fn new(storage_node: Arc<dyn ObjectStorageNode + Send + Sync>) -> Self {
        ObjectService { storage_node }
    }

    pub async fn put(&self, object: Bytes) {
        self.storage_node.put(object).await;
    }
}
