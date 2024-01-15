use anyhow::{bail, Result};
use crypto_common::{
    Key as TKey
};
use aead::{Nonce as TNonce};

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

pub type Crypto = chacha20poly1305::XChaCha20Poly1305;
pub type Key = TKey<Crypto>;
pub type Nonce = TNonce<Crypto>;

use serde::{Serialize, Deserialize};

use base64::prelude::*;


#[derive(Serialize, Deserialize)]
pub struct Recovery {
    pub alg: String,
    pub nonce: String,
    pub cipher: String,
}

pub fn generate_key_recover_blob(root_key: &Key, key: &Key) -> Result<Vec<u8>> {
    let nonce = Crypto::generate_nonce(&mut OsRng);
    let cipher = Crypto::new(&root_key);
    let cipher_result = cipher.encrypt(&nonce, key.as_ref())
        .or_else(|_x| bail!("Encryption error"))?;
    let x = Recovery {
        alg: "XChaCha20Poly1305".to_string(),
        nonce: BASE64_STANDARD.encode(nonce).to_string(),
        cipher: BASE64_STANDARD.encode(cipher_result).to_string(),
    };
    let serialized = serde_json::to_string(&x).unwrap();
    let blob = BASE64_STANDARD.encode(serialized.as_bytes());
    return Ok(blob.as_bytes().to_vec());
}