use super::types::*;
use super::constants::*;

use super::{ToEncryptedStream, utils};

use std::io::Read;
use aead::Aead;
use anyhow::anyhow;
use crypto_common::{KeyInit, KeySizeUser};
use crate::map_anyhow_io;


pub struct EncryptedFileGenerator<'a, R, C> where R: Read, C: KeySizeUser + KeyInit + Aead {
    source: R,
    header: EncryptionFileHeader,
    buffer: Vec<u8>,
    counter: u32,
    key: &'a TKey<C>,
    nonce: TNonce<C>,
    chunk_size: usize,
}

impl<'a, R, C> EncryptedFileGenerator<'a, R, C> where R: Read, C: KeySizeUser + KeyInit + Aead {
    fn new<T: Read>(source: R, key: &'a TKey<C>, header: EncryptionFileHeader) -> Self {
        let chunk_size = header.chunk_size;
        return EncryptedFileGenerator {
            source,
            header,
            buffer: vec![],
            counter: 0,
            key,
            nonce: Default::default(),
            chunk_size: chunk_size as usize,
        };
    }
}

impl<'a, R, C> Read for EncryptedFileGenerator<'a, R, C> where R: Read, C: KeySizeUser + KeyInit + Aead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.counter == 0 {
            self.append_header();
        }
        while self.buffer.len() < buf.len() {
            match self.read_bytes_encrypted() {
                Some(result) => {
                    let mut r = map_anyhow_io!(
                        result,
                        format!("Error encrypting chunk {}", self.counter + 1)
                    )?;
                    self.buffer.append(&mut SEPARATOR.to_vec());
                    self.buffer.append(&mut r);
                }
                None => break,
            }
            self.counter += 1;
            utils::increase_bytes_le(&mut self.nonce);
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

impl<'a, R, C> EncryptedFileGenerator<'a, R, C> where R: Read, C: KeySizeUser + KeyInit + Aead {
    fn append_header(&mut self) {
        //Append file version
        let mut file_version = vec![ENCRYPTED_FILE_VERSION];
        self.buffer.append(&mut file_version);

        //Append header
        let mut header = serde_json::to_string(&self.header).unwrap().to_string();
        self.write_context(&mut header);
    }

    fn write_context(&mut self, context: &mut String) {
        let mut context_length = (context.len() as u16).to_le_bytes().to_vec();
        self.buffer.append(&mut context_length);
        self.buffer.append(&mut context.as_bytes().to_vec());
    }

    pub fn read_bytes_encrypted(&mut self) -> Option<Result<Vec<u8>>> {
        let mut buffer = vec![0u8; self.chunk_size];
        let res = self.source.read(&mut buffer);
        let ret = match res {
            Ok(count) => {
                if count > 0 {
                    Some(crate::encryptor::core::encrypt_chunk::<C>(&buffer[..count].to_vec(), self.key, &self.nonce))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(anyhow!(e))),
        };
        return ret;
    }
}


impl<T: Read> ToEncryptedStream<T> for T {
    type Output<'a, C: KeySizeUser + KeyInit + Aead> = EncryptedFileGenerator<'a, T, C>;
    fn to_encrypted_stream<C: KeySizeUser + KeyInit + Aead>(self, key: &TKey<C>, header: EncryptionFileHeader) ->
    Result<Self::Output<'_, C>>
    {
        return Ok(EncryptedFileGenerator::new::<T>(self, key, header));
    }
}



