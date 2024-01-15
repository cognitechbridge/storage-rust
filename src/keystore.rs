mod serializable;

use std::collections::HashMap;
use aead::rand_core::{CryptoRng, RngCore};
use generic_array::{ArrayLength, GenericArray};

type Key<N> = GenericArray<u8, N>;

#[derive(Debug)]
pub struct KeyStore<N: ArrayLength<u8>> {
    pub root_key: Key<N>,
    pub data_key_map: HashMap<String, Key<N>>,
}

impl<N: ArrayLength<u8>> Default for KeyStore<N> {
    fn default() -> Self {
        return KeyStore {
            data_key_map: Default::default(),
            root_key: Default::default(),
        };
    }
}

impl<N: ArrayLength<u8>> KeyStore<N> {
    pub fn new(key: Key<N>) -> Self <> {
        return KeyStore {
            root_key: key,
            data_key_map: Default::default(),
        };
    }
    pub fn insert(&mut self, key_id: &str, key: Key<N>) {
        self.data_key_map.insert(key_id.to_string(), key);
    }

    pub fn generate_key(mut rng: impl CryptoRng + RngCore) -> Key<N> {
        let mut key = Key::<N>::default();
        rng.fill_bytes(&mut key);
        key
    }

    pub fn generate_store_key(&mut self, key_id: &str, rng: impl CryptoRng + RngCore) -> Key<N> {
        let key = Self::generate_key(rng);
        self.insert(key_id, key.clone());
        key
    }
}
