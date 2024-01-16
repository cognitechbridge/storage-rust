use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use aead::Aead;
use anyhow::{Result};
use crypto_common::{KeyInit, KeySizeUser};
use generic_array::{ArrayLength, GenericArray};
use crate::keystore::KeyStore;
use crate::utils::get_user_path;

type Key<N> = GenericArray<u8, N>;


impl<N: ArrayLength<u8>, C: KeySizeUser<KeySize=N> + KeyInit + Aead> KeyStore<N, C> {
    pub fn persist_key(&self, key_id: &str, key: &Key<N>) -> Result<()> {
        let str = self.serialize_key_pair(key_id, key);
        let mut path = get_user_path()?;
        path.push("key_store");
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        writeln!(file, "{}", str?)?;
        Ok(())
    }
    pub fn load_from_persist(&mut self) -> Result<()> {
        let mut path = get_user_path()?;
        path.push("key_store");
        let mut file = File::open(path).unwrap();
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let str = String::from_utf8(buf)?;
        self.load_from_string(&str)?;
        Ok(())
    }
}