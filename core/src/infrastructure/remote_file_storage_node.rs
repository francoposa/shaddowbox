use std::any::Any;
use std::error::Error;

use async_trait::async_trait;
use reqwest::header::CONTENT_LENGTH;
use reqwest::{Body, Client, Method};

use crate::domain::object::ObjectStripe;
use crate::domain::object_stripe_storage_node::ObjectStripeStorageNode;

pub struct RemoteStorageNode {
    uri: String,
}

impl RemoteStorageNode {
    pub fn new(uri: String) -> Self {
        RemoteStorageNode { uri }
    }
}

#[async_trait]
impl ObjectStripeStorageNode for RemoteStorageNode {
    async fn put_object_stripe(
        &self,
        object_stripe: ObjectStripe,
    ) -> Result<Box<dyn Any>, Box<dyn Error>> {
        let client = Client::new();
        let uri = self.uri.clone() + &"/" + &object_stripe.key;

        let req = client
            .request(Method::PUT, uri)
            .header(CONTENT_LENGTH, object_stripe.bytes.len())
            .body(Body::from(object_stripe.bytes))
            .build()?;

        let resp = client.execute(req).await?;
        Ok(Box::new(resp))
    }
}
