use crate::domain::object::ObjectStripe;
use crate::domain::object_stripe_storage_node::ObjectStripeStorageNode;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Method, StatusCode};
use std::sync::Arc;
use tracing::instrument;

pub struct StorageNodeObjectStripeHandler {
    pub object_stripe_storage: Arc<dyn ObjectStripeStorageNode + Send + Sync>,
}

#[allow(unused)]
#[axum::debug_handler]
#[instrument(skip(handler, body))]
pub async fn put_object_stripe(
    State(handler): State<Arc<StorageNodeObjectStripeHandler>>,
    Path(key): Path<String>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(), (StatusCode, String)> {
    let mut object_stripe = ObjectStripe::new(key, body);
    handler
        .object_stripe_storage
        .put_object_stripe(object_stripe)
        .await;
    Ok(())
}
