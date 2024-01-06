use std::io::{Read, Write};
use crate::encryptor::EncryptedIterator;
use num_bigint::{BigUint};

pub struct EncryptedFileGenerator<T> where T: Read {
    source: EncryptedIterator<T>,
    buffer: Vec<u8>,
    counter: u32,
    chunk_size: usize,
}

impl<T> Read for EncryptedFileGenerator<T> where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.buffer.len() < buf.len() {
            if let Some(result) = self.source.next() {
                self.buffer.append(&mut result.unwrap());
            }
            if self.buffer.is_empty() {
                self.counter += 1;
                return Ok(0);
            }
            if self.counter == 0 {
                self.chunk_size = self.buffer.len();
                let mut size = [0u8;8];
                let bytes = BigUint::from(self.chunk_size).to_bytes_le();
                size[..bytes.len()].copy_from_slice(&bytes);
                self.buffer.splice(0..0, size);
            }
        }
        let len = std::cmp::min(buf.len(), self.buffer.len());
        buf[..len].copy_from_slice(&self.buffer[..len]);
        self.buffer.drain(..len);
        self.counter += 1;
        Ok(len)
    }
}

impl<T> EncryptedFileGenerator<T> where T: Read {
    pub fn new(i: EncryptedIterator<T>) -> Self {
        return EncryptedFileGenerator {
            source: i,
            buffer: vec![],
            counter: 0,
            chunk_size: 0,
        };
    }
}


