use std::sync::Arc;

use crate::domain::object_stripe_storage_node::ObjectStripeStorageNode;

pub struct ReplicationMode {
    pub replication_factor: u32,
}

pub struct StripingConf {
    pub stripe_unit_size_bytes: usize,
}

pub struct ObjectStorageNodeDistributor {
    pub replication: ReplicationMode,
    pub striping: StripingConf,
}

impl ObjectStorageNodeDistributor {
    pub fn select_nodes(
        &self,
        object_storage_nodes: &[Arc<dyn ObjectStripeStorageNode + Send + Sync>],
    ) -> Vec<Arc<dyn ObjectStripeStorageNode + Send + Sync>> {
        let mut selected_nodes = Vec::new();
        for (i, node) in object_storage_nodes.iter().cloned().enumerate() {
            if i < self.replication.replication_factor as usize {
                selected_nodes.push(node);
            } else {
                break;
            }
        }
        return selected_nodes;
    }
}
