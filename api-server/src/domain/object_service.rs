use crate::domain::object_storage_node::ObjectStorageNode;
use std::sync::Arc;

pub struct ObjectService<SN: ObjectStorageNode> {
    storage_node: Arc<SN>,
}

impl<SN> ObjectService<SN>
where
    SN: ObjectStorageNode,
{
    pub fn new(storage_node: Box<SN>) -> Self {
        ObjectService {
            storage_node: Arc::from(storage_node),
        }
    }
}
