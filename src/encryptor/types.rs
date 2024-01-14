use crypto_common::{
    Key as TKey
};
use aead::{Nonce as TNonce};

pub type Result<T> = anyhow::Result<T>;
pub type Crypto = chacha20poly1305::ChaCha20Poly1305;
pub type Key = TKey<Crypto>;
pub type Nonce = TNonce<Crypto>;
pub type Context = Vec<u8>;