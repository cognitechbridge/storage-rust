use std::io::Read;
use serde::{Deserialize, Serialize};

pub use types::*;

mod encrypt_file;
mod decrypt_file;
mod encrypt_iterator;
mod utils;
mod core;
mod constants;
pub mod types;
mod file_header;


pub trait ToEncryptedStream<'a, Y> where Y: Read {
    fn to_encrypted_stream(self, key: &'a Key, header: EncryptionFileHeader) -> Result<Y>;
}

pub trait ToPlainStream<'a, Y> where Y: Read {
    fn to_plain_stream(self, key: &'a Key) -> Y;
}




