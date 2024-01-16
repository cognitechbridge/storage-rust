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
pub struct GeneratedKey<'a, N: ArrayLength<u8>> {
    pub key: &'a Key<N>,
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
        let key_ref = self
            .insert_get(&key_id.to_string(), key)
            .ok_or(anyhow!("Error inserting key to store"))?;
        let res = GeneratedKey {
            key: key_ref,
            recovery_blob: blob,
        };
        Ok(res)
    }
}