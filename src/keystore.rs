mod serialize;
mod recovery;
mod generate_key;
mod persistence;

use anyhow::Result;
use std::collections::HashMap;
use std::marker::PhantomData;
use crate::common::{Crypto, Key, Nonce};
use crate::keystore::persistence::KeyStorePersist;

pub struct KeyStore<C: Crypto> {
    root_key: Key<C>,
    recovery_key: Option<Key<C>>,
    persist: KeyStorePersist,
    alg: PhantomData<C>,
}

impl<C: Crypto> Default for KeyStore<C> {
    fn default() -> Self {
        let persist = KeyStorePersist::new().unwrap();
        KeyStore {
            root_key: Default::default(),
            recovery_key: Default::default(),
            persist,
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

    pub fn init(&mut self) -> Result<()> {
        self.persist.init()?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, key_id: &str, key: Key<C>) -> Result<()> {
        self.persist_key(key_id, key)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get(&self, key_id: &str) -> Result<Option<Key<C>>> {
        let res = self.persist.get_key(key_id)?;
        match res {
            None => Ok(None),
            Some((nonce, key)) => {
                let key = self.deserialize_key_pair(&nonce, &key)?.clone();
                Ok(Some(key))
            }
        }
    }
}
