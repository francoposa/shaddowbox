use crate::domain::object::Object;
use async_trait::async_trait;
use std::any::Any;
use std::error::Error;

#[async_trait]
pub trait ObjectStorageNode {
    async fn put(&self, object: Object) -> Result<Box<dyn Any>, Box<dyn Error>>;
}
