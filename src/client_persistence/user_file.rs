use crate::common::{
    utils::get_user_path
};
use std::fs::File;
use std::io::Read;
use crate::client_persistence::ClientPersistence;
use anyhow::Result;

pub struct ClientFolderPersistence {}

impl ClientFolderPersistence {
}

impl ClientPersistence for ClientFolderPersistence {
    fn load_client_config(config_key: String) -> Result<Option<String>> {
        let mut path = get_user_path()?;
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