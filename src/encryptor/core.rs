use aead::Aead;
use anyhow::{bail};
use crypto_common::KeyInit;
use super::{Crypto, Key, Nonce, Result};

pub fn encrypt(plain: &Vec<u8>, key: &Key, nonce: &Nonce) -> Result<Vec<u8>> {
    let cipher = Crypto::new(&key);
    let cipher_result = cipher.encrypt(&nonce, plain.as_ref())
        .or_else(|_x| bail!("Encryption error"))?;
    return Ok(cipher_result);
}

pub fn decrypt(encrypted: &Vec<u8>, key: &Key, nonce: &Nonce) -> Result<Vec<u8>> {
    let cipher = Crypto::new(&key);
    let plain_result = cipher.decrypt(&nonce, encrypted.as_ref())
        .or_else(|_x| bail!("Decryption error"))?;
    return Ok(plain_result);
}