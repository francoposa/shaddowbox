use std::sync::Arc;

use axum::routing::put;
use axum::Router;
use shaddowbox_core::application::server::object_stripe_handler;
use shaddowbox_core::application::server::object_stripe_handler::StorageNodeObjectStripeHandler;
use shaddowbox_core::infrastructure::local_file_storage_node::LocalFileStorageNode;
use tokio::fs;
use tracing::level_filters::LevelFilter;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

const SERVICE_NAME: &str = "shaddowbox-storage-node";
const TMP_FILE_DIR: &str = "tmp/files";

#[tokio::main]
async fn main() {
    // stdout/stderr log layer for non-tracing logs to be collected into ElasticSearch or similar
    let std_stream_bunyan_format_layer =
        BunyanFormattingLayer::new(SERVICE_NAME.into(), std::io::stderr)
            .with_filter(LevelFilter::INFO);

    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(std_stream_bunyan_format_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    fs::create_dir_all(TMP_FILE_DIR)
        .await
        .expect("failed to create temp file directory");

    let local_storage_node = LocalFileStorageNode::new(String::from(TMP_FILE_DIR));
    let object_stripe_storage = Arc::from(local_storage_node);

    let object_stripe_handler = StorageNodeObjectStripeHandler {
        object_stripe_storage,
    };

    let app = Router::new()
        .route("/*key", put(object_stripe_handler::put_object_stripe))
        .with_state(Arc::from(object_stripe_handler));

    let host = "0.0.0.0";
    let port = "8082";
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let server = axum::serve(listener, app);

    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
}
