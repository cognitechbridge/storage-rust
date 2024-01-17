use super::{
    *,
    Key,
    KeyStore
};
use aead::rand_core::{CryptoRng, RngCore};
use generic_array::ArrayLength;
use uuid::Uuid;
use anyhow::Result;

#[derive(Debug)]
pub struct GeneratedKey<N: ArrayLength<u8>> {
    pub key: Key<N>,
    pub recovery_blob: String,
}

impl<N: ArrayLength<u8>, C: Crypto<KeySize=N>> KeyStore<N, C> {
    pub fn generate_key_pair(
        &mut self,
        key_id: &Uuid,
        rng: impl CryptoRng + RngCore + Clone)
        -> Result<GeneratedKey<N>>
    {
        let key = C::generate_key(rng.clone());
        let nonce = C::generate_nonce(rng.clone());
        let blob = self.generate_recovery_blob(&key, &nonce)?;
        self.insert(&key_id.to_string(), key.clone())?;
        let res = GeneratedKey {
            key,
            recovery_blob: blob,
        };
        Ok(res)
    }
}