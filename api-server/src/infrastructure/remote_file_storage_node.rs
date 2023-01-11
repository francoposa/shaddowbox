use crate::domain::object::Object;
use crate::domain::object_storage_node::ObjectStorageNode;
use async_trait::async_trait;
use hyper::{Body, Client, Method, Request, Uri};
use std::any::Any;
use std::borrow::Borrow;
use std::error::Error;

pub struct RemoteFileStorageNode {
    uri: String,
}

impl RemoteFileStorageNode {
    pub fn new(uri: String) -> Self {
        RemoteFileStorageNode { uri }
    }
}

#[async_trait]
impl ObjectStorageNode for RemoteFileStorageNode {
    async fn put(&self, object: Object) -> Result<Box<dyn Any>, Box<dyn Error>> {
        let client = Client::new();
        let uri = self.uri.clone() + &"/" + &object.key;

        let req = Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::from(object.bytes))
            .expect("request builder");

        let resp = client.request(req).await?;
        Ok(Box::new(resp))
    }
}
