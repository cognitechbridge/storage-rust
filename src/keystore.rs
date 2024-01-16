mod serialize;
mod recovery;
mod generate_key;
mod persistence;

pub use recovery::DataKeyRecoveryGenerator;
use anyhow::Result;
use std::collections::HashMap;
use std::marker::PhantomData;
use aead::Aead;
use aead::rand_core::{CryptoRng, RngCore};
use crypto_common::{KeyInit, KeySizeUser};
use generic_array::{ArrayLength, GenericArray};

type Key<N> = GenericArray<u8, N>;

pub struct KeyStore<N: ArrayLength<u8>, C: KeySizeUser<KeySize=N> + KeyInit + Aead> {
    root_key: Key<N>,
    data_key_map: HashMap<String, Key<N>>,
    alg: PhantomData<C>
}

impl<N: ArrayLength<u8>, C: KeySizeUser<KeySize=N> + KeyInit + Aead> Default for KeyStore<N, C> {
    fn default() -> Self {
        return KeyStore {
            data_key_map: Default::default(),
            root_key: Default::default(),
            alg: PhantomData
        };
    }
}

impl<N: ArrayLength<u8>, C: KeySizeUser<KeySize=N> + KeyInit + Aead> KeyStore<N,C> {
    pub fn new(key: Key<N>) -> Self <> {
        return KeyStore {
            root_key: key,
            ..Default::default()
        };
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, key_id: &str, key: Key<N>) -> Result<()> {
        self.persist_key(key_id, &key)?;
        self.data_key_map.insert(key_id.to_string(), key);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn generate_rnd_key(mut rng: impl CryptoRng + RngCore) -> Key<N> {
        let mut key = Key::<N>::default();
        rng.fill_bytes(&mut key);
        key
    }

    #[allow(dead_code)]
    pub fn get(&self, key_id: &str) -> Option<&Key<N>> {
        let key = self.data_key_map.get(key_id);
        key
    }
}
