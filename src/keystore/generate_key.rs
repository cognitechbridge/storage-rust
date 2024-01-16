use aead::Aead;
use aead::rand_core::{CryptoRng, RngCore};
use generic_array::ArrayLength;
use uuid::Uuid;
use anyhow::{anyhow, Result};
use chacha20poly1305::XChaCha20Poly1305;
use crypto_common::{KeyInit, KeySizeUser};
use crypto_common::typenum::U32;
use super::{DataKeyRecoveryGenerator, Key, KeyStore};

#[derive(Debug)]
pub struct GeneratedKey<N: ArrayLength<u8>> {
    pub key: Key<N>,
    pub recovery_blob: String,
}

impl<N: ArrayLength<u8>> KeyStore<N> {
    pub fn generate_key_pair<C>(&mut self, key_id: &Uuid, rng: impl CryptoRng + RngCore)
                                -> Result<GeneratedKey<N>>
        where C: KeySizeUser<KeySize=N> + KeyInit + Aead
    {
        let key = Self::generate_rnd_key(rng);
        let blob = DataKeyRecoveryGenerator::<C>::new(&self.root_key)
            .with_uuid_nonce(&key, key_id)?;
        self.insert(&key_id.to_string(), key.clone());
        let res = GeneratedKey {
            key,
            recovery_blob: blob,
        };
        Ok(res)
    }
}