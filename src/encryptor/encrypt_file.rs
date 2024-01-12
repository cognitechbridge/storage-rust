use anyhow::{anyhow, Result};

use super::types::{IdContext, Key};
use super::constants::{ENCRYPTED_FILE_VERSION, HEADER_RESERVE_LENGTH, SEPARATOR};

use super::encrypt_iterator::EncryptedIterator;
use super::ToEncryptedStream;

use std::io::Read;
use num_bigint::{BigUint};
use crate::map_anyhow_io;


pub struct EncryptedFileGenerator<'a, T> where T: Read {
    source: EncryptedIterator<'a, T>,
    id_context: IdContext,
    buffer: Vec<u8>,
    counter: u32,
    chunk_size: usize,
}

impl<'a, T: Read> EncryptedFileGenerator<'a, T> {
    fn new(iterator: EncryptedIterator<'a, T>, id_context: IdContext) -> Self {
        return EncryptedFileGenerator {
            source: iterator,
            id_context,
            buffer: vec![],
            counter: 0,
            chunk_size: 0,
        };
    }
}

impl<'a, T: Read> Read for EncryptedFileGenerator<'a, T> {
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
                        self.append_header();
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

impl<'a, T: Read> EncryptedFileGenerator<'a, T> {
    fn append_header(&mut self) {
        //Append file version
        let mut file_version = vec![ENCRYPTED_FILE_VERSION];
        self.buffer.append(&mut file_version);

        //Append id context
        let mut bytes = self.id_context.to_vec();
        self.buffer.append(&mut bytes);

        //Append chunk size
        let mut size = [0u8; HEADER_RESERVE_LENGTH];
        let bytes = BigUint::from(self.chunk_size).to_bytes_le();
        size[..bytes.len()].copy_from_slice(&bytes);
        self.buffer.append(&mut size.to_vec());
    }
}

impl<'a, T: Read> ToEncryptedStream<'a, EncryptedFileGenerator<'a, T>> for T {
    fn to_encrypted_stream(self, key: &'a Key, id_context: IdContext, chunk_size: usize)
                           -> Result<EncryptedFileGenerator<'a, T>> {
        if id_context.len() != 36 {
            return Err(anyhow!("Invalid id context length."));
        }
        let iterator = EncryptedIterator::new(self, key, chunk_size);
        return Ok(EncryptedFileGenerator::new(iterator, id_context));
    }
}


