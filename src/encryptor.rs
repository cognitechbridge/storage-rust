use std::io::Read;

pub use types::*;
pub use crate::common::Crypto;

mod encrypt_file;
mod decrypt_file;
mod utils;
mod core;
mod constants;
pub mod types;
mod file_header;

pub trait ToEncryptedStream<T: Read> {
    type Output<'a, C: Crypto>: Read;
    fn to_encrypted_stream<C: Crypto>(self, key: &TKey<C>, header: EncryptionFileHeader<C>) ->
    Result<Self::Output<'_, C>>;
}

pub trait ToPlainStream<Y: Read> {
    type Output<'a, C: Crypto>: Read;
    fn to_plain_stream<C: Crypto>(self, key: &TKey<C>) -> Self::Output<'_, C>;
}

