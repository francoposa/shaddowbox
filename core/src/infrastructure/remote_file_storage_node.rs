use crate::domain::object::ObjectStripe;
use crate::domain::object_stripe_storage_node::ObjectStripeStorageNode;
use async_trait::async_trait;
use hyper::{Body, Client, Method, Request};
use std::any::Any;
use std::error::Error;

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

        let req = Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .header("content-length", object_stripe.bytes.len())
            .body(Body::from(object_stripe.bytes))
            .expect("request builder");

        let resp = client.request(req).await?;
        Ok(Box::new(resp))
    }
}
