use aead::Aead;
use aead::rand_core::{CryptoRng, RngCore};
use generic_array::ArrayLength;
use uuid::Uuid;
use anyhow::{anyhow, Result};
use crypto_common::{KeyInit, KeySizeUser};
use super::{DataKeyRecoveryGenerator, Key, KeyStore};

#[derive(Debug)]
pub struct GeneratedKey<N: ArrayLength<u8>> {
    pub key: Key<N>,
    pub recovery_blob: String,
}

impl<N: ArrayLength<u8>, C: KeySizeUser<KeySize=N> + KeyInit + Aead> KeyStore<N, C> {
    pub fn generate_key_pair(
        &mut self,
        key_id: &Uuid,
        rng: impl CryptoRng + RngCore + Clone)
        -> Result<GeneratedKey<N>>
        where C: KeySizeUser<KeySize=N> + KeyInit + Aead
    {
        let key = C::generate_key(rng.clone());
        let nonce = C::generate_nonce(rng.clone());
        let recovery_key = self.recovery_key.as_ref().ok_or(anyhow!("Recovery key is not stored"))?;
        let blob = DataKeyRecoveryGenerator::<C>::new(recovery_key)
            .generate(&key, &nonce)?;
        self.insert(&key_id.to_string(), key.clone())?;
        let res = GeneratedKey {
            key,
            recovery_blob: blob,
        };
        Ok(res)
    }
}