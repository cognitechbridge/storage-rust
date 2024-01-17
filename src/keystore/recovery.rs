use crate::utils::*;

use aead::{Aead, AeadCore};

use anyhow::{anyhow, bail, Result};
use crypto_common::{KeyInit, KeySizeUser};
use serde::{Serialize, Deserialize};

use base64::prelude::*;
use generic_array::{ArrayLength, GenericArray};
use crate::keystore::KeyStore;
use crate::utils::as_array::AsArray;


type Key<N> = GenericArray<u8, N>;

#[derive(Serialize, Deserialize)]
pub enum RecoveryVersion {
    V1
}


#[derive(Serialize, Deserialize)]
pub struct Recovery {
    pub version: RecoveryVersion,
    pub alg: String,
    pub nonce: String,
    pub cipher: String,
}

impl<N: ArrayLength<u8>, C: KeySizeUser<KeySize=N> + KeyInit + Aead> KeyStore<N, C> {

    pub fn get_recovery_key(&self) -> Option<&crate::keystore::Key<N>> {
        self.recovery_key.as_ref()
    }

    pub fn set_recover_key(&mut self, recovery_key: crate::keystore::Key<N>) -> Result<()> {
        if self.loaded == false {
            bail!("Cannot update recovery key: Store not loaded");
        }
        if self.get_recovery_key().is_some() {
            bail!("Cannot update recovery key: Key exist");
        }
        self.recovery_key = Some(recovery_key);
        self.persist_recovery_key()?;
        Ok(())
    }

    pub fn generate_recovery_blob(
        &self,
        key: &Key<N>,
        nonce: &impl AsArray<<C as AeadCore>::NonceSize>,
    ) -> Result<String> {
        let recovery_key = self.get_recovery_key().ok_or(anyhow!("No recovery key is stored."))?;
        let cipher = C::new(recovery_key);
        let nonce = nonce.as_array();
        let cipher_result = cipher.encrypt(&nonce, key.as_ref())
            .or_else(|_x| bail!("Encryption error"))?;
        let x = Recovery {
            version: RecoveryVersion::V1,
            alg: type_name_of::<C>()?,
            nonce: BASE64_STANDARD.encode(nonce).to_string(),
            cipher: BASE64_STANDARD.encode(cipher_result).to_string(),
        };
        let serialized = serde_json::to_string(&x)?;

        let blob = BASE64_STANDARD.encode(serialized.as_bytes());
        Ok(blob)
    }
}

