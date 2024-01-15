use super::types::*;
use super::constants::*;
use super::{utils, core, ToPlainStream};

use std::io::Read;
use aead::{Aead, AeadCore};
use anyhow::{anyhow, bail};
use crypto_common::{KeyInit, KeySizeUser};
use crate::map_anyhow_io;

use generic_array::typenum::Unsigned;

pub struct ReaderDecryptor<'a, T, C> where T: Read, C: KeySizeUser + KeyInit + Aead {
    source: T,
    key: &'a TKey<C>,
    nonce: TNonce<C>,
    buffer: Vec<u8>,
    chunk_size: usize,
    chunk_counter: usize,
}

impl<T: Read> ToPlainStream<T> for T {
    type Output<'a, C: KeySizeUser + KeyInit + Aead> = ReaderDecryptor<'a, T, C>;
    fn to_plain_stream<C: KeySizeUser + KeyInit + Aead>(self, key: &TKey<C>) -> Self::Output<'_, C> {
        return ReaderDecryptor {
            source: self,
            key,
            nonce: Default::default(),
            buffer: vec![],
            chunk_size: 0,
            chunk_counter: 0,
        };
    }
}

impl<'a, T, C> Read for ReaderDecryptor<'a, T, C> where T: Read, C: KeySizeUser + KeyInit + Aead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.chunk_size == 0 {
            let header = map_anyhow_io!(
                read_file_header(&mut self.source),
                "Error reading file header"
            )?;
            self.chunk_size = header.chunk_size as usize + <C as AeadCore>::TagSize::to_usize();
        }
        while self.buffer.len() < buf.len() {
            map_anyhow_io!(
                read_chunk_header(&mut self.source),
                "Error reading chunk header"
            )?;
            let mut buffer = vec![0u8; self.chunk_size];
            let bytes_read = match self.source.read(&mut buffer) {
                Ok(0) => break,
                Ok(size) => size,
                Err(_e) => 0,
            };
            self.chunk_counter += 1;
            let mut decrypted_data = map_anyhow_io!(
                core::decrypt_chunk::<C>(&buffer[..bytes_read].to_vec(),&self.key,&self.nonce),
                format!("Error decrypting chunk {}", self.chunk_counter)
            )?;
            self.buffer.append(&mut decrypted_data);
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

fn read_file_header(source: &mut impl Read) -> Result<EncryptionFileHeader> {
    //Read file version
    read_file_version(source)?;

    //Read file header
    let header = read_header(source)?;

    return Ok(header);
}

fn read_file_version(source: &mut (impl Read + Sized)) -> Result<()> {
    let mut buffer = [0u8; 1];
    source.read(&mut buffer)?;
    if buffer[0] != 1u8 {
        return Err(anyhow!("File version invalid"));
    }
    return Ok(());
}

fn read_header(source: &mut (impl Read + Sized)) -> Result<EncryptionFileHeader> {
    //Read context size
    let mut buffer_2 = [0u8; 2];
    source.read(&mut buffer_2)?;
    let context_size = u16::from_le_bytes(buffer_2);

    //Read context
    let mut buffer_context = vec![0; context_size as usize];
    source.read(&mut buffer_context)?;

    let file_header = serde_json::from_slice(buffer_context.as_slice()).unwrap();

    Ok(file_header)
}

fn read_chunk_header(source: &mut impl Read) -> Result<()> {
    let mut small_buffer = [0u8; 4];
    let size = source.read(&mut small_buffer)?;
    if size == 0 { return Ok(()); }
    return if size == 4 && small_buffer.iter().all(|&x| x == 0u8)
    {
        Ok(())
    } else {
        bail!("Chunk header is not valid: {:?}", small_buffer);
    };
}