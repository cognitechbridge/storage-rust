use std::io::{Read, Write};
use crate::encryptor::EncryptedIterator;

pub struct EncryptedFileGenerator<T> where T: Read {
    source: EncryptedIterator<T>,
    buffer: Vec<u8>,
    counter: u32,
    pub chunk_size: usize,
}

impl<T> Read for EncryptedFileGenerator<T> where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.buffer.len() < buf.len() {
            if let Some(result) = self.source.next() {
                self.buffer.append(&mut result.unwrap());
                println!("{:x?}", &self.buffer);
            }
            if self.buffer.is_empty() {
                return Ok(0);
            }
        }

        let len = std::cmp::min(buf.len(), self.buffer.len());
        buf[..len].copy_from_slice(&self.buffer[..len]);
        self.buffer.drain(..len);
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


