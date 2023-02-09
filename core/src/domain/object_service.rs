use crate::domain::object::{ObjectStream, ObjectStripe};
use crate::domain::object_stripe_storage_node::ObjectStripeStorageNode;
use crate::domain::object_stripe_storage_node_distribution::ObjectStorageNodeDistributor;
use bytes::Bytes;
use std::any::Any;

use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tracing::instrument;

pub const BLOCK_SIZE: usize = 1024 * 1024;

pub struct ObjectService {
    storage_nodes: Vec<Arc<dyn ObjectStripeStorageNode + Send + Sync>>,
    storage_node_distributor: ObjectStorageNodeDistributor,
}

impl ObjectService {
    pub fn new(
        storage_nodes: Vec<Arc<dyn ObjectStripeStorageNode + Send + Sync>>,
        storage_node_distributor: ObjectStorageNodeDistributor,
    ) -> Self {
        ObjectService {
            storage_nodes,
            storage_node_distributor,
        }
    }

    #[instrument(skip_all)]
    pub async fn put_object(
        &self,
        mut object: ObjectStream,
    ) -> Result<Box<dyn Any>, Box<dyn Error>> {
        let selected_nodes = self
            .storage_node_distributor
            .select_nodes(&self.storage_nodes);

        let mut buf_position = 0;
        let mut i = 0;
        while buf_position + BLOCK_SIZE <= object.len_bytes {
            let mut buf = vec![0; BLOCK_SIZE];
            object
                .stream_reader
                .read_exact(&mut buf)
                .await
                .expect("TODO: panic message");

            let object_stripe = ObjectStripe {
                key: String::from(object.key.clone()) + &i.to_string(),
                bytes: Bytes::from(buf.clone()),
            };
            let storage_node = selected_nodes.first().unwrap().as_ref();
            match storage_node.put_object_stripe(object_stripe).await {
                Ok(_) => (),
                Err(err) => return Err(err),
            };
            buf_position += BLOCK_SIZE;
            i += 1;
        }
        Ok(Box::from(()))
    }
}
