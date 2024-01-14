use anyhow::{anyhow, Result};

use super::types::*;
use super::constants::*;

use super::encrypt_iterator::EncryptedIterator;
use super::{ToEncryptedStream};

use std::io::Read;
use num_bigint::{BigUint};
use crate::map_anyhow_io;


pub struct EncryptedFileGenerator<'a, T> where T: Read {
    source: EncryptedIterator<'a, T>,
    header: EncryptionFileHeader,
    buffer: Vec<u8>,
    counter: u32,
    chunk_size: usize,
}

impl<'a, T: Read> EncryptedFileGenerator<'a, T> {
    fn new(iterator: EncryptedIterator<'a, T>, header: EncryptionFileHeader) -> Self {
        return EncryptedFileGenerator {
            source: iterator,
            header,
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

        //Append header
        let mut header = serde_json::to_string(&self.header).unwrap().to_string();
        self.write_context(&mut header);
    }

    fn write_context(&mut self, mut context: &mut String) {
        let mut context_length = (context.len() as u16).to_le_bytes().to_vec();
        self.buffer.append(&mut context_length);
        self.buffer.append(&mut context.as_bytes().to_vec());
    }
}


impl<'a, T: Read> ToEncryptedStream<'a, EncryptedFileGenerator<'a, T>> for T {
    fn to_encrypted_stream(self, key: &'a Key, header: EncryptionFileHeader) -> Result<EncryptedFileGenerator<'a, T>>
    {
        let iterator = EncryptedIterator::new(self, key, header.chunk_size as usize);

        return Ok(EncryptedFileGenerator::new(iterator, header));
    }
}


