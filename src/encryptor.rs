use crypto_common::{
    Key as TKey
};

use aead::{
    Nonce as TNonce,
    Aead,
    KeyInit
};

pub mod encrypt_file;
pub mod decrypt_file;
pub mod encrypt_iterator;
mod utils;
mod core;


pub type Result<T> = anyhow::Result<T>;
pub type Crypto = chacha20poly1305::ChaCha20Poly1305;
pub type Key = TKey<Crypto>;
pub type Nonce = TNonce<Crypto>;
