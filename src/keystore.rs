mod serialize;
mod recovery;
mod generate_key;
mod persistence;

use anyhow::Result;
use std::collections::HashMap;
use std::marker::PhantomData;
use crate::common::{Crypto, Key, Nonce};

pub struct KeyStore<C: Crypto> {
    root_key: Key<C>,
    recovery_key: Option<Key<C>>,
    data_key_map: HashMap<String, Key<C>>,
    loaded: bool,
    alg: PhantomData<C>,
}

impl<C: Crypto> Default for KeyStore<C> {
    fn default() -> Self {
        KeyStore {
            data_key_map: Default::default(),
            root_key: Default::default(),
            recovery_key: Default::default(),
            loaded: false,
            alg: PhantomData,
        }
    }
}

impl<C: Crypto> KeyStore<C> {
    pub fn new(root_key: Key<C>) -> Self <> {
        KeyStore {
            root_key,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, key_id: &str, key: Key<C>) -> Result<()> {
        self.persist_key(key_id, &key)?;
        self.data_key_map.insert(key_id.to_string(), key);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get(&self, key_id: &str) -> Option<&Key<C>> {
        let key = self.data_key_map.get(key_id);
        key
    }
}
