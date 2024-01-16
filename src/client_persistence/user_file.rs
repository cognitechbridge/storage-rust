use std::fs::{create_dir_all, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::client_persistence::ClientPersistence;
use anyhow::{anyhow, Result};

pub struct ClientFolderPersistence {}

impl ClientFolderPersistence {
    fn get_user_path() -> Result<PathBuf> {
        let mut path = dirs::home_dir().ok_or(anyhow!("Cannot find user home"))?;
        path.push(".ctb");
        if !path.exists() {
            create_dir_all(&path)?;
        }
        Ok(path)
    }
}

impl ClientPersistence for ClientFolderPersistence {
    fn load_client_config(config_key: String) -> Result<Option<String>> {
        let mut path = Self::get_user_path()?;
        path.push(config_key);
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Some(contents))
    }

    fn save_client_config(config_key: String) -> Result<String> {
        Ok("".to_string())
    }
}