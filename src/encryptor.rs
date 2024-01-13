use std::io::Read;
use types::*;

mod encrypt_file;
mod decrypt_file;
mod encrypt_iterator;
mod utils;
mod core;
mod constants;
pub mod types;

pub trait ToEncryptedStream<'a, Y> where Y: Read {
    fn to_encrypted_stream(self, key: &'a Key, id_context: impl ToString, chunk_size: usize) -> Result<Y>;
}

pub trait ToPlainStream<'a, Y> where Y: Read {
    fn to_plain_stream(self, key: &'a Key) -> Y;
}




