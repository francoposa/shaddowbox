use crate::domain::object_storage_node::ObjectStorageNode;
use async_trait::async_trait;
use bytes::Bytes;
use std::any::Any;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct LocalFileStorageNode {
    file_dir: String,
}

impl LocalFileStorageNode {
    pub fn new(file_dir: String) -> Self {
        LocalFileStorageNode { file_dir }
    }
}

#[async_trait]
impl ObjectStorageNode for LocalFileStorageNode {
    async fn put(&self, object: Arc<Bytes>) -> Result<Box<dyn Any>, Box<dyn Error>> {
        let path = Path::new(&self.file_dir).join("file");
        let mut file = match File::create(path).await {
            Ok(file) => file,
            Err(err) => return Err(Box::new(err)),
        };

        let mut buffer = match Arc::try_unwrap(object) {
            Ok(buffer) => buffer,
            Err(_) => {
                return Err(Box::from(String::from(
                    "failed to get exclusive reference to buffer",
                )))
            }
        };
        return match file.write_all_buf(&mut buffer).await {
            Ok(_) => match file.sync_all().await {
                Ok(_) => Ok(Box::new(())),
                Err(err) => Err(Box::new(err)),
            },
            Err(err) => Err(Box::new(err)),
        };
    }
}
