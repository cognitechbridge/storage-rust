use std::io::Read;
pub use types::*;
use crate::encryptor::encrypt_file::EncryptedFileGenerator;


mod encrypt_file;
mod decrypt_file;
mod utils;
mod core;
mod constants;
pub mod types;
mod file_header;

pub struct Encryptor {
    pub client_id: String,
    pub chunk_size: u64,
}

impl Encryptor {
    pub fn new(client_id: String, chunk_size: u64) -> Self {
        Self {
            client_id,
            chunk_size,
        }
    }
    pub fn encrypt<'a, C: Crypto, R: Read>(&'a self, source: R, file_id: String, key: &'a Key<C>, recovery: String) -> Result<EncryptedFileGenerator<R, C>> {
        let header = EncryptionFileHeader {
            client_id: self.client_id.clone(),
            chunk_size: self.chunk_size,
            file_id,
            recovery,
            ..Default::default()
        };
        Ok(EncryptedFileGenerator::new::<R>(source, key, header))
    }
}

pub trait ToPlainStream<Y: Read> {
    type Output<'a, C: Crypto>: Read;
    fn to_plain_stream<C: Crypto>(self, key: &Key<C>) -> Self::Output<'_, C>;
}

