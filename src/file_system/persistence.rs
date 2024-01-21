use anyhow::Result;

pub trait FileSystemPersist {
    fn save_path(&self, path: &str, key: &str) -> Result<()>;
    fn get_path(&self, path: &str) -> Result<Option<String>>;
}


