use std::io::Read;
use crate::encryptor::EncryptedIterator;
use num_bigint::{BigUint};

const SEPARATOR_LENGTH: usize = 4;
const HEADER_LENGTH: usize = 4;
const SEPARATOR: [u8; SEPARATOR_LENGTH] = [0u8; SEPARATOR_LENGTH];

pub struct EncryptedFileGenerator<T> where T: Read {
    source: EncryptedIterator<T>,
    buffer: Vec<u8>,
    counter: u32,
    chunk_size: usize,
}

impl<T> Read for EncryptedFileGenerator<T> where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        while self.buffer.len() < buf.len() {
            match self.source.next() {
                Some(result) => {
                    let mut r = result.unwrap();
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

impl<T: Read> EncryptedIterator<T> {
    pub fn to_encrypted_file_generator(self) -> EncryptedFileGenerator<T> {
        return EncryptedFileGenerator {
            source: self,
            buffer: vec![],
            counter: 0,
            chunk_size: 0,
        };
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


