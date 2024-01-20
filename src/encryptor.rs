use std::io::Read;
use std::marker::PhantomData;
pub use types::*;
use crate::encryptor::decrypt_file::ReaderDecryptor;
use crate::encryptor::encrypt_file::EncryptedFileGenerator;


mod encrypt_file;
mod decrypt_file;
mod utils;
mod core;
mod constants;
pub mod types;
mod file_header;

pub struct Encryptor<C: Crypto> {
    pub client_id: String,
    pub chunk_size: u64,
    pub alg: PhantomData<C>,
}

impl<C: Crypto> Encryptor<C> {
    pub fn new(client_id: String, chunk_size: u64) -> Self {
        Self {
            client_id,
            chunk_size,
            alg: PhantomData,
        }
    }
    pub fn encrypt<'a, R: Read>(&'a self, source: R, file_id: String, key: &'a Key<C>, recovery: String) -> Result<EncryptedFileGenerator<R, C>> {
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

pub struct Decryptor<C: Crypto> {
    pub alg: PhantomData<C>,
}

impl<C: Crypto> Decryptor<C> {
    pub fn new() -> Self <> {
        Decryptor {
            alg: Default::default(),
        }
    }
    pub fn decrypt<'a, R: Read>(&'a self, key: &'a Key<C>, source: R) -> Result<ReaderDecryptor<R, C>> {
        Ok(ReaderDecryptor::new(key, source))
    }
}

