use aead::{Aead, OsRng};
use super::utils::*;

use anyhow::{bail, Result};
use crypto_common::{Key as TKey, KeyInit, KeySizeUser};


use serde::{Serialize, Deserialize};

use base64::prelude::*;
use crate::map_anyhow_io;


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

pub fn generate_key_recover_blob<C: KeySizeUser + KeyInit + Aead, DC: KeySizeUser>(
    root_key: &TKey<C>, key: &TKey<DC>,
) -> Result<String> {
    let nonce = C::generate_nonce(&mut OsRng);
    let cipher = C::new(&root_key);
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