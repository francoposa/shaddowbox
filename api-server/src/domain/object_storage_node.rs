use async_trait::async_trait;
use bytes::Bytes;
use std::any::Any;
use std::error::Error;

#[async_trait]
pub trait ObjectStorageNode {
    async fn put(&self, bytes: Bytes) -> Result<Box<dyn Any>, Box<dyn Error>>;
}
