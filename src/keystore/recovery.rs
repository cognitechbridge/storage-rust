use super::{
    Crypto,
    KeyStore,
    Key,
};
use crate::common::{
    utils::type_name_of,
};

use aead::{AeadCore};

use anyhow::{anyhow, bail, Result};
use serde::{Serialize, Deserialize};

use base64::prelude::*;
use crate::common::AsGenericArray;

const RECOVERY_KEY: &str = "---------------RECOVERY-------------";

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

impl<C: Crypto> KeyStore<C> {
    pub fn get_recovery_key(&self) -> Option<Key<C>> {
        return self.get(RECOVERY_KEY).expect("Error getting recovery key");
    }

    pub fn set_recover_key(&mut self, recovery_key: Key<C>) -> Result<()> {
        if self.get_recovery_key().is_some() {
            bail!("Cannot update recovery key: Key exist");
        }
        self.recovery_key = Some(recovery_key.clone());
        self.persist_key(RECOVERY_KEY,&recovery_key)?;
        Ok(())
    }

    pub fn generate_recovery_blob(
        &self,
        key: &Key<C>,
        nonce: &impl AsGenericArray<<C as AeadCore>::NonceSize>,
    ) -> Result<String> {
        let recovery_key = self.get_recovery_key().ok_or(anyhow!("No recovery key is stored."))?;
        let cipher = C::new(&recovery_key);
        let nonce = nonce.as_generic_array();
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

