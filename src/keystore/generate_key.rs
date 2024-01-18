use super::{
    *,
    Key,
    KeyStore
};
use aead::rand_core::{CryptoRng, RngCore};
use uuid::Uuid;
use anyhow::Result;

#[derive(Debug)]
pub struct GeneratedKey<C: Crypto> {
    pub key: Key<C>,
    pub recovery_blob: String,
}

impl<C: Crypto> KeyStore<C> {
    pub fn generate_key_pair(
        &mut self,
        key_id: &Uuid,
        rng: impl CryptoRng + RngCore + Clone)
        -> Result<GeneratedKey<C>>
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