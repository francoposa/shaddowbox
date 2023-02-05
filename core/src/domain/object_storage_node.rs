use crate::domain::object::ObjectBlock;
use async_trait::async_trait;
use std::any::Any;
use std::error::Error;

#[async_trait]
pub trait ObjectStorageNode {
    async fn put(&self, object: ObjectBlock) -> Result<Box<dyn Any>, Box<dyn Error>>;
}
