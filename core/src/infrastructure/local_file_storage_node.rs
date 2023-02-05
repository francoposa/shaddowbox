use crate::domain::object::ObjectBlock;
use crate::domain::object_storage_node::ObjectStorageNode;
use async_trait::async_trait;
use std::any::Any;
use std::borrow::Borrow;
use std::error::Error;
use std::fs;
use std::path::Path;
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
    async fn put(&self, object: ObjectBlock) -> Result<Box<dyn Any>, Box<dyn Error>> {
        let path = Path::new(&self.file_dir).join(object.key);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?
        }

        let mut file = match File::create(path).await {
            Ok(file) => file,
            Err(err) => return Err(Box::new(err)),
        };

        return match file.write_all(object.bytes.borrow()).await {
            Ok(_) => match file.sync_all().await {
                Ok(_) => Ok(Box::new(())),
                Err(err) => Err(Box::new(err)),
            },
            Err(err) => Err(Box::new(err)),
        };
    }
}
