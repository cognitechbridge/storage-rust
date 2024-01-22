use super::{Crypto, KeyStore, Key};
use crate::common::{
    utils::type_name_of,
};

use aead::{AeadCore};
use aead::rand_core::CryptoRngCore;

use anyhow::{anyhow, bail, Result};
use serde::{Serialize, Deserialize};

use base64::prelude::*;
use uuid::Uuid;
use crate::common::ToGenericArray;

const RECOVERY_TAG: &str = "RECOVERY";

#[derive(Serialize, Deserialize)]
pub enum RecoveryVersion {
    V1
}

#[derive(Debug)]
pub struct GeneratedKey<C: Crypto> {
    pub key: Key<C>,
    pub recovery_blob: String,
}

#[derive(Serialize, Deserialize)]
pub struct Recovery {
    pub version: RecoveryVersion,
    pub alg: String,
    pub nonce: String,
    pub cipher: String,
    pub id: String,
}

impl<C: Crypto> KeyStore<C> {
    pub fn get_recovery_key(&self) -> Option<(String, Key<C>)> {
        self.get_with_tag(RECOVERY_TAG).expect("Error getting recovery key")
    }
    pub fn generate_recovery_blob(
        &self,
        key: &Key<C>,
        nonce: impl ToGenericArray<<C as AeadCore>::NonceSize>,
    ) -> Result<String> {
        let (recovery_id, recovery_key) = self.get_recovery_key().ok_or(anyhow!("No recovery key is stored."))?;
        let cipher = C::new(&recovery_key);
        let nonce = nonce.to_generic_array();
        let cipher_result = cipher.encrypt(&nonce, key.as_ref())
            .or_else(|_x| bail!("Encryption error"))?;
        let x = Recovery {
            version: RecoveryVersion::V1,
            alg: type_name_of::<C>()?,
            id: recovery_id,
            nonce: BASE64_STANDARD.encode(nonce).to_string(),
            cipher: BASE64_STANDARD.encode(cipher_result).to_string(),
        };
        let serialized = serde_json::to_string(&x)?;

        let blob = BASE64_STANDARD.encode(serialized.as_bytes());
        Ok(blob)
    }
    pub fn set_recover_key(&mut self, id: &str, key: &Key<C>) -> Result<()> {
        if self.get_recovery_key().is_some() {
            bail!("Cannot update recovery key: Key exist");
        }
        self.persist_key(id, key, RECOVERY_TAG)?;
        Ok(())
    }
    pub fn generate_key_pair(
        &mut self,
        key_id: &Uuid,
        rng: impl CryptoRngCore + Clone)
        -> Result<GeneratedKey<C>>
    {
        let key = C::generate_key(rng.clone());
        let nonce = C::generate_nonce(rng);
        let blob = self.generate_recovery_blob(&key, nonce)?;
        self.insert(&key_id.to_string(), &key)?;
        let res = GeneratedKey {
            key,
            recovery_blob: blob,
        };
        Ok(res)
    }
}

