use async_trait::async_trait;
use bytes::Buf;
use std::any::Any;
use std::error::Error;
use std::sync::Arc;

#[async_trait]
pub trait ObjectStorageNode {
    async fn put(
        &self,
        mut object: Arc<&mut (impl Buf + Send + Sync)>,
    ) -> Result<Box<dyn Any>, Box<dyn Error>>;
}
