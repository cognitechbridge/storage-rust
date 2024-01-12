use std::io::Read;
use types::Key;

mod encrypt_file;
mod decrypt_file;
mod encrypt_iterator;
mod utils;
mod core;
mod constants;
pub mod types;

pub trait ToEncryptedStream<'a, Y> where Y: Read {
    fn to_encrypted_stream(self, key: &'a Key, chunk_size: usize) -> Y;
}

pub trait ToPlainStream<'a, Y> where Y: Read {
    fn to_plain_stream(self, key: &'a Key) -> Y;
}




