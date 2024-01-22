use std::sync::Arc;
use anyhow::Result;


pub trait FileSystem {
    fn save_path(&self, path: &str, key: &str) -> Result<()>;
    fn get_path(&self, path: &str) -> Result<Option<String>>;
}

pub struct PersistFileSystem {
    persist: Arc<dyn FileSystem>,
}


impl PersistFileSystem {
    pub fn new(persist: Arc<dyn FileSystem>) -> Self <> {
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