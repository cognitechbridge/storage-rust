use super::types::*;
use super::{utils, core};

use std::io::Read;
use aead::Aead;
use anyhow::anyhow;
use crypto_common::{KeyInit, KeySizeUser};

pub struct EncryptedIterator<'a, T, C> where T: Read, C: KeySizeUser + KeyInit + Aead {
    source: T,
    key: &'a TKey<C>,
    nonce: TNonce<C>,
    pub chunk_size: usize,
}

impl<'a, T: Read, C: KeySizeUser + KeyInit + Aead> EncryptedIterator<'a, T, C> {
    pub fn new(source: T, key: &'a TKey<C>, chunk_size: usize) -> EncryptedIterator<T, C> {
        return EncryptedIterator {
            source,
            key,
            nonce: Default::default(),
            chunk_size,
        };
    }
    pub fn read_bytes_encrypted(&mut self, size: usize) -> Option<Result<Vec<u8>>> {
        let mut buffer = vec![0u8; size];
        let res = self.source.read(&mut buffer);
        let ret = match res {
            Ok(count) => {
                if count > 0 {
                    Some(core::encrypt_chunk::<C>(&buffer[..count].to_vec(), &self.key, &self.nonce))
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

impl<'a, T: Read, C: KeySizeUser + KeyInit + Aead> Iterator for EncryptedIterator<'a, T, C> {
    type Item = Result<Vec<u8>>;
    fn next(&mut self) -> Option<Self::Item> {
        return self.read_bytes_encrypted(self.chunk_size);
    }
}