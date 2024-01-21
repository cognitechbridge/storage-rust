use super::{
    *,
    constants::{SEPARATOR, ENCRYPTED_FILE_VERSION},
    Crypto, Key, Nonce,
    utils,
};

use std::io::Read;
use anyhow::anyhow;
use crate::map_anyhow_io;


pub struct EncryptedFileGenerator<'a, R, C> where C: Crypto {
    source: R,
    header: EncryptionFileHeader<C>,
    key: &'a Key<C>,
    buffer: Vec<u8>,
    nonce: Nonce<C>,
    chunk_counter: u32,
    chunk_size: usize,
}

impl<'a, R, C> EncryptedFileGenerator<'a, R, C> where C: Crypto {
    pub fn new(source: R, key: &'a Key<C>, header: EncryptionFileHeader<C>) -> Self {
        let chunk_size = header.chunk_size;
        EncryptedFileGenerator {
            source,
            header,
            buffer: vec![],
            chunk_counter: 0,
            key,
            nonce: Default::default(),
            chunk_size: chunk_size as usize,
        }
    }
}

impl<'a, R, C> Read for EncryptedFileGenerator<'a, R, C> where R: Read, C: Crypto {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.chunk_counter == 0 {
            map_anyhow_io!(self.append_header(),"Error appending header")?;
        }
        while self.buffer.len() < buf.len() {
            match self.read_bytes_encrypted() {
                Some(result) => {
                    let mut r = map_anyhow_io!(
                        result,
                        format!("Error encrypting chunk {}", self.chunk_counter + 1)
                    )?;
                    self.buffer.append(&mut SEPARATOR.to_vec());
                    self.buffer.append(&mut r);
                }
                None => break,
            }
            self.chunk_counter += 1;
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

impl<'a, R, C> EncryptedFileGenerator<'a, R, C> where R: Read, C: Crypto {
    fn append_header(&mut self) -> Result<()> {
        //Append file version
        let mut file_version = vec![ENCRYPTED_FILE_VERSION];
        self.buffer.append(&mut file_version);

        //Append header
        let mut header = serde_json::to_string(&self.header)?.to_string();
        self.write_context(&mut header);
        Ok(())
    }

    fn write_context(&mut self, context: &mut String) {
        let mut context_length = (context.len() as u16).to_le_bytes().to_vec();
        self.buffer.append(&mut context_length);
        self.buffer.append(&mut context.as_bytes().to_vec());
    }

    pub fn read_bytes_encrypted(&mut self) -> Option<Result<Vec<u8>>> {
        let mut buffer = vec![0u8; self.chunk_size];
        let res = self.source.read(&mut buffer);
        match res {
            Ok(count) => {
                if count > 0 {
                    Some(super::core::encrypt_chunk::<C>(&buffer[..count].to_vec(), self.key, &self.nonce))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(anyhow!(e))),
        }
    }
}



