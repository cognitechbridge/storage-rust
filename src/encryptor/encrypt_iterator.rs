use std::io::Read;
use anyhow::anyhow;
use super::{utils::increase_bytes_le, core, Key, Nonce, Result};

pub struct EncryptedIterator<T> where T: Read {
    source: T,
    key: Key,
    nonce: Nonce,
    pub chunk_size: usize,
}

pub trait ToEncryptedIterator<T> where T: Read {
    fn to_encrypted_iterator(self, key: Key, chunk_size: usize) -> EncryptedIterator<T>;
}

impl<T: Read> ToEncryptedIterator<T> for T {
    fn to_encrypted_iterator(self, key: Key, chunk_size: usize) -> EncryptedIterator<T> {
        return EncryptedIterator {
            source: self,
            key,
            nonce: Nonce::default(),
            chunk_size,
        };
    }
}


impl<T: Read> EncryptedIterator<T> {
    pub fn read_bytes_encrypted(&mut self, size: usize) -> Option<Result<Vec<u8>>> {
        let mut buffer = vec![0u8; size];
        let res = self.source.read(&mut buffer);
        let ret = match res {
            Ok(count) => {
                if count > 0 {
                    Some(core::encrypt(&buffer[..count].to_vec(), &self.key, &self.nonce))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(anyhow!(e))),
        };
        increase_bytes_le(&mut self.nonce);
        return ret;
    }
}

impl<T> Iterator for EncryptedIterator<T> where T: Read {
    type Item = Result<Vec<u8>>;
    fn next(&mut self) -> Option<Self::Item> {
        return self.read_bytes_encrypted(self.chunk_size);
    }
}