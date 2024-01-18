use std::io::Read;

pub use types::*;


mod encrypt_file;
mod decrypt_file;
mod utils;
mod core;
mod constants;
pub mod types;
mod file_header;

pub trait ToEncryptedStream<T: Read> {
    type Output<'a, C: Crypto>: Read;
    fn to_encrypted_stream<C: Crypto>(self, key: &Key<C>, header: EncryptionFileHeader<C>) ->
    Result<Self::Output<'_, C>>;
}

pub trait ToPlainStream<Y: Read> {
    type Output<'a, C: Crypto>: Read;
    fn to_plain_stream<C: Crypto>(self, key: &Key<C>) -> Self::Output<'_, C>;
}

