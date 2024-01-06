use std::io::Read;
use num_bigint::{BigUint};
use num_traits::ToPrimitive;
use crate::encryptor;
use crate::encryptor::{Key, Nonce};

pub struct ReaderDecryptor<T> where T: Read {
    source: T,
    key: Key,
    nonce: Nonce,
    buffer: Vec<u8>,
    chunk_size: usize,
}

pub trait AsReaderDecryptor<T> where T: Read {
    fn to_reader_decryptor(self, key:Key, nonce: Nonce) -> ReaderDecryptor<T>;
}

impl<T: Read> AsReaderDecryptor<T> for T {
    fn to_reader_decryptor(self, key:Key, nonce: Nonce) -> ReaderDecryptor<T> {
        return ReaderDecryptor {
            source: self,
            key,
            nonce,
            buffer: vec![],
            chunk_size: 0,
        };
    }
}

impl<T> Read for ReaderDecryptor<T> where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.chunk_size == 0 {
            let mut small_buffer = [0u8; 4];
            self.source.read(&mut small_buffer).unwrap();
            self.chunk_size = BigUint::from_bytes_le(&mut small_buffer).to_u64().unwrap() as usize;
        }
        while self.buffer.len() < buf.len() {
            let mut small_buffer = [0u8; 4];
            self.source.read(&mut small_buffer).unwrap();

            let mut buffer = vec![0u8; self.chunk_size];
            let bytes_read = match self.source.read(&mut buffer) {
                Ok(0) => break,
                Ok(size) => size,
                Err(_e) => 0,
            };
            let mut decrypted_data = encryptor::decrypt(
                &buffer[..bytes_read].to_vec(),
                &self.key,
                &self.nonce).unwrap();
            self.buffer.append(&mut decrypted_data);
            encryptor::increase_bytes_le(&mut self.nonce);
        }
        if self.buffer.is_empty() {
            return Ok(0);
        }

        let len = std::cmp::min(buf.len(), self.buffer.len());
        buf[..len].copy_from_slice(&self.buffer[..len]);
        self.buffer.drain(..len);
        Ok(len)
    }
}