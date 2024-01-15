pub use crypto_common::{
    Key as TKey
};
pub use aead::{Nonce as TNonce};

pub use super::file_header::EncryptionFileHeader;

pub type Result<T> = anyhow::Result<T>;