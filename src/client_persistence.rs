use anyhow::{Result};

pub mod user_file;

pub trait ClientPersistence {
    fn load_client_config(config_key: String) -> Result<Option<String>>;
    fn save_client_config(config_key: String) -> Result<String>;
}