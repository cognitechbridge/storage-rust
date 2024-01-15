use aead::{Aead, OsRng};
use super::utils::*;

use anyhow::{bail, Result};
use crypto_common::{Key as TKey, KeyInit, KeySizeUser};
use serde::{Serialize, Deserialize};

use base64::prelude::*;
use generic_array::{ArrayLength, GenericArray};
use uuid::Uuid;
use crate::encryptor::TNonce;

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

pub struct DataKeyRecoveryGenerator<'a, C> where C: KeySizeUser + KeyInit + Aead {
    root_key: &'a TKey<C>,
}

impl<'a, C> DataKeyRecoveryGenerator<'a, C> where C: KeySizeUser + KeyInit + Aead {
    pub fn new(root_key: &'a TKey<C>) -> Self {
        return DataKeyRecoveryGenerator {
            root_key
        };
    }
    pub fn with_nonce<N: ArrayLength<u8>>(
        &self,
        key: &Key<N>,
        nonce: TNonce<C>,
    ) -> Result<String> {
        let cipher = C::new(self.root_key);
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
        return Ok(blob);
    }

    pub fn with_uuid<N: ArrayLength<u8>>(
        &self,
        key: &Key<N>,
        uuid: &Uuid,
    ) -> Result<String> {
        let mut nonce: TNonce<C> = Default::default();
        nonce[..16].copy_from_slice(uuid.as_bytes());
        self.with_nonce(key, nonce)
    }

    pub fn with_rand<N: ArrayLength<u8>>(
        &self,
        key: &Key<N>,
    ) -> Result<String> {
        let nonce = C::generate_nonce(&mut OsRng);
        self.with_nonce(key, nonce)
    }
}

