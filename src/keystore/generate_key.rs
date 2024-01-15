use aead::Aead;
use aead::rand_core::{CryptoRng, RngCore};
use generic_array::ArrayLength;
use uuid::Uuid;
use anyhow::Result;
use chacha20poly1305::XChaCha20Poly1305;
use crypto_common::{KeyInit, KeySizeUser};
use crypto_common::typenum::U32;
use super::{DataKeyRecoveryGenerator, Key, KeyStore};

#[derive(Debug)]
pub struct GeneratedKey<N: ArrayLength<u8>> {
    key: Key<N>,
    recovery_blob: String,
}

impl<N: ArrayLength<u8>> KeyStore<N> {
    #[allow(dead_code)]
    pub fn generate_store_key(&mut self, key_id: &str, rng: impl CryptoRng + RngCore) -> Key<N> {
        let key = Self::generate_key(rng);
        self.insert(key_id, key.clone());
        key
    }
    pub fn generate_stored_key<C>(&mut self, key_id: &Uuid, rng: impl CryptoRng + RngCore)
                                  -> Result<GeneratedKey<N>>
        where C: KeySizeUser<KeySize=N> + KeyInit + Aead
    {
        let key = Self::generate_key(rng);
        self.insert(&key_id.to_string(), key.clone());
        let blob = DataKeyRecoveryGenerator::<C>::new(&self.root_key)
            .with_uuid(&key, key_id)?;
        let res = GeneratedKey {
            key,
            recovery_blob: blob,
        };
        Ok(res)
    }
}