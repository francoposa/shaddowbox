use async_trait::async_trait;
use bytes::Bytes;
use std::any::Any;
use std::error::Error;
use std::sync::Arc;

#[async_trait]
pub trait ObjectStorageNode {
    async fn put(&self, object: Arc<Bytes>) -> Result<Box<dyn Any>, Box<dyn Error>>;
}
