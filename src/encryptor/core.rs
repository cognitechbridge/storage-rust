use super::{
    *,
    Crypto,
    Key,
    Nonce
};
use anyhow::{bail};


pub fn encrypt_chunk<C: Crypto>(plain: &Vec<u8>, key: &Key<C>, nonce: &Nonce<C>) -> Result<Vec<u8>> {
    let cipher = C::new(key);
    let cipher_result = cipher.encrypt(nonce, plain.as_ref())
        .or_else(|_x| bail!("Encryption error"))?;
    Ok(cipher_result)
}

pub fn decrypt_chunk<C: Crypto>(encrypted: &Vec<u8>, key: &Key<C>, nonce: &Nonce<C>) -> Result<Vec<u8>> {
    let cipher = C::new(key);
    let plain_result = cipher.decrypt(nonce, encrypted.as_ref())
        .or_else(|_x| bail!("Decryption error"))?;
    Ok(plain_result)
}