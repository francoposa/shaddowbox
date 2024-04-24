use std::any::Any;
use std::error::Error;

use async_trait::async_trait;

use crate::domain::object::ObjectStripe;

#[async_trait]
pub trait ObjectStripeStorageNode {
    async fn put_object_stripe(
        &self,
        object_stripe: ObjectStripe,
    ) -> Result<Box<dyn Any>, Box<dyn Error>>;
}
