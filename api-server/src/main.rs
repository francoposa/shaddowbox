mod application;
mod domain;
mod infrastructure;

use crate::application::server::object_handler::APIObjectHandler;
use crate::domain::object_service::ObjectService;

use crate::infrastructure::local_file_storage_node::LocalFileStorageNode;
use application::server::object_handler;
use axum::{
    routing::{get, post},
    Router,
};
use hyper::server::Server;

use std::sync::Arc;
use tokio::fs;
use tracing::level_filters::LevelFilter;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{prelude::*, Registry};

const SERVICE_NAME: &str = "shaddowbox-api-server";
const TMP_FILE_DIR: &str = "tmp/files";

#[tokio::main]
async fn main() {
    // stdout/stderr log layer for non-tracing logs to be collected into ElasticSearch or similar
    let std_stream_bunyan_format_layer =
        BunyanFormattingLayer::new(SERVICE_NAME.into(), std::io::stderr)
            .with_filter(LevelFilter::INFO);

    let subscriber = Registry::default().with(std_stream_bunyan_format_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    fs::create_dir_all(TMP_FILE_DIR)
        .await
        .expect("failed to create temp file directory");

    let object_storage_node = LocalFileStorageNode::new(String::from(TMP_FILE_DIR));
    let object_service = ObjectService::new(Arc::from(object_storage_node));

    let api_object_handler = Arc::new(APIObjectHandler {
        object_service: Arc::from(object_service),
    });

    let app = Router::new()
        .route("/*key", post(object_handler::put))
        .with_state(api_object_handler);

    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
