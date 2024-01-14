use aead::Aead;
use anyhow::{bail};
use crypto_common::{KeyInit, KeySizeUser};
use super::types::*;


pub fn encrypt_chunk<C: KeySizeUser + KeyInit + Aead>(plain: &Vec<u8>, key: &TKey<C>, nonce: &TNonce<C>) -> Result<Vec<u8>> {
    let cipher = C::new(key);
    let cipher_result = cipher.encrypt(&nonce, plain.as_ref())
        .or_else(|_x| bail!("Encryption error"))?;
    return Ok(cipher_result);
}

pub fn decrypt_chunk<C: KeySizeUser + KeyInit + Aead>(encrypted: &Vec<u8>, key: &TKey<C>, nonce: &TNonce<C>) -> Result<Vec<u8>> {
    let cipher = C::new(&key);
    let plain_result = cipher.decrypt(&nonce, encrypted.as_ref())
        .or_else(|_x| bail!("Decryption error"))?;
    return Ok(plain_result);
}