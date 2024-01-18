use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use anyhow::{Result};
use crate::common::{Crypto, Key};
use crate::keystore::KeyStore;
use crate::utils::get_user_path;

impl<C: Crypto> KeyStore<C> {
    fn get_persist_path() -> Result<PathBuf> {
        let mut path = get_user_path()?;
        path.push("key_store");
        Ok(path)
    }
    fn append_to_file(str: &str) -> Result<()>{
        let path = Self::get_persist_path()?;
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        writeln!(file, "{}", str)?;
        Ok(())
    }
    pub fn persist_recovery_key(&self) -> Result<()> {
        let str = self.serialize_recovery_key()?;
        Self::append_to_file(&str)?;
        Ok(())
    }
    pub fn persist_key(&self, key_id: &str, key: &Key<C>) -> Result<()> {
        let str = self.serialize_key_pair(key_id, key)?;
        Self::append_to_file(&str)?;
        Ok(())
    }
    pub fn load_from_persist(&mut self) -> Result<()> {
        let path = Self::get_persist_path()?;
        if !path.exists() {
            return Ok(())
        }
        let mut file = File::open(path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let str = String::from_utf8(buf)?;
        self.load_from_string(&str)?;
        Ok(())
    }
}