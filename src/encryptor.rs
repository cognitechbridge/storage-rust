use std::io::Read;
use aead::Aead;
use crypto_common::{KeyInit, KeySizeUser};

pub use types::*;

mod encrypt_file;
mod decrypt_file;
mod utils;
mod core;
mod constants;
pub mod types;
mod file_header;


pub trait ToEncryptedStream<T: Read> {
    type Output<'a, C: KeySizeUser + KeyInit + Aead>: Read;
    fn to_encrypted_stream<C: KeySizeUser + KeyInit + Aead>(self, key: &TKey<C>, header: EncryptionFileHeader) ->
    Result<Self::Output<'_, C>>;
}

pub trait ToPlainStream<Y: Read> {
    type Output<'a, C: KeySizeUser + KeyInit + Aead>: Read;
    fn to_plain_stream<C: KeySizeUser + KeyInit + Aead>(self, key: &TKey<C>) -> Self::Output<'_, C>;
}




