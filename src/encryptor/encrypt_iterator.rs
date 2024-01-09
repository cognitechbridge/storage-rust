use super::types::{Key, Nonce, Result};
use super::{utils, core};

use std::io::Read;
use anyhow::anyhow;



pub struct EncryptedIterator<T> where T: Read {
    source: T,
    key: Key,
    nonce: Nonce,
    pub chunk_size: usize,
}

impl<T: Read> EncryptedIterator<T> {
    pub fn new(source: T, key: Key, chunk_size: usize) -> EncryptedIterator<T> {
        return EncryptedIterator {
            source,
            key,
            nonce: Nonce::default(),
            chunk_size,
        };
    }
    pub fn read_bytes_encrypted(&mut self, size: usize) -> Option<Result<Vec<u8>>> {
        let mut buffer = vec![0u8; size];
        let res = self.source.read(&mut buffer);
        let ret = match res {
            Ok(count) => {
                if count > 0 {
                    Some(core::encrypt_chunk(&buffer[..count].to_vec(), &self.key, &self.nonce))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(anyhow!(e))),
        };
        utils::increase_bytes_le(&mut self.nonce);
        return ret;
    }
}

impl<T: Read> Iterator for EncryptedIterator<T> {
    type Item = Result<Vec<u8>>;
    fn next(&mut self) -> Option<Self::Item> {
        return self.read_bytes_encrypted(self.chunk_size);
    }
}