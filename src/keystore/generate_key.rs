use aead::rand_core::{CryptoRng, RngCore};
use generic_array::ArrayLength;
use uuid::Uuid;
use anyhow::Result;
use chacha20poly1305::XChaCha20Poly1305;
use crypto_common::typenum::U32;
use super::{DataKeyRecoveryGenerator, Key, KeyStore};

pub struct GeneratedKey<N: ArrayLength<u8>> {
    key: Key<N>,
    recovery_blob: String,
}

impl KeyStore<U32> {
    #[allow(dead_code)]
    pub fn generate_store_key(&mut self, key_id: &str, rng: impl CryptoRng + RngCore) -> Key<U32> {
        let key = Self::generate_key(rng);
        self.insert(key_id, key.clone());
        key
    }
    pub fn generate_stored_key(&mut self, key_id: &Uuid, rng: impl CryptoRng + RngCore) -> Result<GeneratedKey<U32>> {
        let key = Self::generate_key(rng);
        self.insert(&key_id.to_string(), key.clone());
        let blob = DataKeyRecoveryGenerator::<XChaCha20Poly1305>::new(&self.root_key)
            .with_uuid(&key, key_id)?;
        let res = GeneratedKey {
            key,
            recovery_blob: blob,
        };
        Ok(res)
    }
}