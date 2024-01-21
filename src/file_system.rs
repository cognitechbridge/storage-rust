use std::sync::Arc;
use crate::common::Crypto;
use anyhow::Result;

mod persistence;

pub use persistence::FileSystemPersist;


pub struct PersistFileSystem {
    persist: Arc<dyn FileSystemPersist>,
}


impl PersistFileSystem {
    pub fn new(persist: Arc<dyn FileSystemPersist>) -> Self <> {
        PersistFileSystem {
            persist,
        }
    }
    pub fn save_path(&self, id: &str, path: &str) -> Result<()> {
        self.persist.save_path(path, id)
    }

    pub fn get_path(&self, path: &str) -> Result<Option<String>> {
        self.persist.get_path(path)
    }
}