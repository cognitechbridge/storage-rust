use super::types::*;
use super::constants::*;

use super::encrypt_iterator::EncryptedIterator;
use super::{ToEncryptedStream};

use std::io::Read;
use aead::Aead;
use crypto_common::{KeyInit, KeySizeUser};
use crate::map_anyhow_io;


pub struct EncryptedFileGenerator<I> where I: Iterator<Item=Result<Vec<u8>>> {
    source: I,
    header: EncryptionFileHeader,
    buffer: Vec<u8>,
    counter: u32,
    chunk_size: usize,
}

impl<I> EncryptedFileGenerator<I> where I: Iterator<Item=Result<Vec<u8>>> {
    fn new<T: Read, C: KeySizeUser + KeyInit + Aead>(iterator: I, header: EncryptionFileHeader) -> Self {
        return EncryptedFileGenerator {
            source: iterator,
            header,
            buffer: vec![],
            counter: 0,
            chunk_size: 0,
        };
    }
}

impl<I> Read for EncryptedFileGenerator<I> where I: Iterator<Item=Result<Vec<u8>>> {
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

impl<I> EncryptedFileGenerator<I> where I: Iterator<Item=Result<Vec<u8>>> {
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


impl<T: Read> ToEncryptedStream<T> for T {
    type Output<'a, C: KeySizeUser + KeyInit + Aead> = EncryptedFileGenerator<EncryptedIterator<'a, T, C>>;
    fn to_encrypted_stream<C: KeySizeUser + KeyInit + Aead>(self, key: &TKey<C>, header: EncryptionFileHeader) ->
    Result<Self::Output<'_, C>>
    {
        let iterator = EncryptedIterator::new(self, key, header.chunk_size as usize);
        return Ok(EncryptedFileGenerator::new::<T, C>(iterator, header));
    }
}


