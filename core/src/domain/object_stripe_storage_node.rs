use crate::domain::object::ObjectStripe;
use async_trait::async_trait;
use std::any::Any;
use std::error::Error;

#[async_trait]
pub trait ObjectStripeStorageNode {
    async fn put_object_stripe(
        &self,
        object_stripe: ObjectStripe,
    ) -> Result<Box<dyn Any>, Box<dyn Error>>;
}
