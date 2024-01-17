mod serialize;
mod recovery;
mod generate_key;
mod persistence;

use anyhow::Result;
use std::collections::HashMap;
use std::marker::PhantomData;
use generic_array::{ArrayLength, GenericArray};
use crate::common::Crypto;

type Key<N> = GenericArray<u8, N>;

pub struct KeyStore<N: ArrayLength<u8>, C: Crypto<KeySize=N>> {
    root_key: Key<N>,
    recovery_key: Option<Key<N>>,
    data_key_map: HashMap<String, Key<N>>,
    loaded: bool,
    alg: PhantomData<C>,
}

impl<N: ArrayLength<u8>, C: Crypto<KeySize=N>> Default for KeyStore<N, C> {
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

impl<N: ArrayLength<u8>, C: Crypto<KeySize=N>> KeyStore<N, C> {
    pub fn new(root_key: Key<N>) -> Self <> {
        KeyStore {
            root_key,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, key_id: &str, key: Key<N>) -> Result<()> {
        self.persist_key(key_id, &key)?;
        self.data_key_map.insert(key_id.to_string(), key);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get(&self, key_id: &str) -> Option<&Key<N>> {
        let key = self.data_key_map.get(key_id);
        key
    }
}
