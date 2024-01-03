use crypto_common::{
    Key as TKey
};

use aead::{
    Nonce as TNonce,
    Aead,
    KeyInit
};

use std::fs::File;
use std::io::{self, Read};

pub type Result<T> = core::result::Result<T, aead::Error>;
type Crypto = chacha20poly1305::ChaCha20Poly1305;
pub type Key = TKey<Crypto>;
pub type Nonce = TNonce<Crypto>;

fn encrypt_file(filename: &str, key:&Key, nonce:&Nonce) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let encrypted = encrypt(contents, &key, &nonce).unwrap();
    Ok(encrypted)
}

pub fn encrypt(plain:Vec<u8>, key:&Key, nonce:&Nonce) -> Result<Vec<u8>> {

  let cipher = Crypto::new(&key);
  let cipher_result = cipher.encrypt(&nonce, plain.as_ref());

  return  cipher_result;
}

pub fn decrypt(encrypted:Vec<u8>, key:&Key, nonce:&Nonce) -> Result<Vec<u8>> {

  let cipher = Crypto::new(&key);
  let plain_result = cipher.decrypt(&nonce, encrypted.as_ref());

  return  plain_result;
}
