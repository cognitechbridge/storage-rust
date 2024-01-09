use super::types::Key;
use super::constants::{HEADER_LENGTH, SEPARATOR};

use super::encrypt_iterator::EncryptedIterator;
use super::ToEncryptedStream;

use std::io::Read;
use num_bigint::{BigUint};
use crate::map_anyhow_io;


pub struct EncryptedFileGenerator<T> where T: Read {
    source: EncryptedIterator<T>,
    buffer: Vec<u8>,
    counter: u32,
    chunk_size: usize,
}

impl<T: Read> EncryptedFileGenerator<T> {
    fn new(iterator: EncryptedIterator<T>) -> Self {
        return EncryptedFileGenerator {
            source: iterator,
            buffer: vec![],
            counter: 0,
            chunk_size: 0,
        };
    }
}

impl<T: Read> Read for EncryptedFileGenerator<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        while self.buffer.len() < buf.len() {
            match self.source.next() {
                Some(result) => {
                    let mut r = map_anyhow_io!(
                        result,
                        format!("Error encrypting chunk {}", self.counter + 1)
                    )?;
                    if self.counter == 0 {
                        self.chunk_size = r.len();
                        let mut size = [0u8; HEADER_LENGTH];
                        let bytes = BigUint::from(self.chunk_size).to_bytes_le();
                        size[..bytes.len()].copy_from_slice(&bytes);
                        self.buffer.append(&mut size.to_vec());
                    }
                    self.buffer.append(&mut SEPARATOR.to_vec());
                    self.counter += 1;
                    self.buffer.append(&mut r)
                }
                None => break,
            }
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

impl<T: Read> ToEncryptedStream<EncryptedFileGenerator<T>> for T {
    fn to_encrypted_stream(self, key: Key, chunk_size: usize) -> EncryptedFileGenerator<T> {
        let iterator = EncryptedIterator::new(self, key, chunk_size);
        return EncryptedFileGenerator::new(iterator);
    }
}


