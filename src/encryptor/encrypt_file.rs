use anyhow::{anyhow, Result};

use super::types::{Context, Key};
use super::constants::{ENCRYPTED_FILE_VERSION, HEADER_RESERVE_LENGTH, SEPARATOR};

use super::encrypt_iterator::EncryptedIterator;
use super::ToEncryptedStream;

use std::io::Read;
use num_bigint::{BigUint};
use crate::map_anyhow_io;


pub struct EncryptedFileGenerator<'a, T> where T: Read {
    source: EncryptedIterator<'a, T>,
    file_id_context: Context,
    client_id_context: Context,
    buffer: Vec<u8>,
    counter: u32,
    chunk_size: usize,
}

impl<'a, T: Read> EncryptedFileGenerator<'a, T> {
    fn new(iterator: EncryptedIterator<'a, T>, file_id_context: Context, client_id_context: Context) -> Self {
        return EncryptedFileGenerator {
            source: iterator,
            file_id_context,
            client_id_context,
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

        //Append client id context
        let mut context = self.client_id_context.clone();
        self.write_context(&mut context);

        //Append file id context
        let mut context = self.file_id_context.clone();
        self.write_context(&mut context);

        //Append chunk size
        let mut size = [0u8; HEADER_RESERVE_LENGTH];
        let bytes = BigUint::from(self.chunk_size).to_bytes_le();
        size[..bytes.len()].copy_from_slice(&bytes);
        self.buffer.append(&mut size.to_vec());
    }

    fn write_context(&mut self, mut context: &mut Vec<u8>) {
        let mut context_length = vec![context.len() as u8];
        self.buffer.append(&mut context_length);
        self.buffer.append(&mut context);
    }
}


impl<'a, T: Read> ToEncryptedStream<'a, EncryptedFileGenerator<'a, T>> for T {
    fn to_encrypted_stream(self,
                           key: &'a Key,
                           client_id_context: impl ToString,
                           file_id_context: impl ToString, chunk_size: usize)
                           -> Result<EncryptedFileGenerator<'a, T>>
    {
        let file_id_vec = file_id_context.to_string().into_bytes();
        let client_id_vec = client_id_context.to_string().into_bytes();

        if file_id_vec.len() > 255 {
            return Err(anyhow!("Invalid id context string length. It should be less than 255 bytes."));
        }
        let iterator = EncryptedIterator::new(self, key, chunk_size);

        return Ok(EncryptedFileGenerator::new(iterator, file_id_vec, client_id_vec));
    }
}


