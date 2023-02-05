use axum::{routing::put, Router};
use hyper::server::Server;

use shaddowbox_core::application::server::object_handler;
use shaddowbox_core::application::server::object_handler::APIObjectHandler;
use shaddowbox_core::domain::object_service::{ObjectService, BLOCK_SIZE};
use shaddowbox_core::domain::object_storage_node::ObjectStorageNode;
use shaddowbox_core::domain::object_storage_node_distribution::{
    ObjectStorageNodeDistributor, ReplicationMode, StripingConf,
};
use shaddowbox_core::infrastructure::remote_file_storage_node::RemoteFileStorageNode;
use std::sync::Arc;
use tokio::fs;
use tracing::level_filters::LevelFilter;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{prelude::*, Registry};

const SERVICE_NAME: &str = "shaddowbox-api-server";
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

    let remote_storage_node = RemoteFileStorageNode::new(String::from("http://127.0.0.1:8081"));
    let arc_storage_node: Arc<dyn ObjectStorageNode + Send + Sync> = Arc::from(remote_storage_node);
    let object_service = ObjectService::new(
        vec![arc_storage_node],
        ObjectStorageNodeDistributor {
            replication: ReplicationMode {
                replication_factor: 1,
            },
            striping: StripingConf {
                stripe_unit_size_bytes: BLOCK_SIZE,
            },
        },
    );

    let api_object_handler = Arc::new(APIObjectHandler {
        object_service: Arc::from(object_service),
    });

    let app = Router::new()
        .route("/*key", put(object_handler::put_object))
        .with_state(api_object_handler);

    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
