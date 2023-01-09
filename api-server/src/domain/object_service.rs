use crate::domain::object::Object;
use crate::domain::object_storage_node::ObjectStorageNode;
use std::sync::Arc;
use tracing::error;

pub struct ObjectService {
    storage_node: Arc<dyn ObjectStorageNode + Send + Sync>,
}

impl ObjectService {
    pub fn new(storage_node: Arc<dyn ObjectStorageNode + Send + Sync>) -> Self {
        ObjectService { storage_node }
    }

    pub async fn put_object(&self, object: Object) {
        match self.storage_node.put(object).await {
            Ok(_) => (),
            Err(err) => error!(err),
        };
    }
}
