use super::{
    *,
    utils,
    core,
    Crypto, Key, Nonce,
};

use std::io::Read;
use aead::AeadCore;
use anyhow::bail;
use crate::map_anyhow_io;

use generic_array::typenum::Unsigned;

pub struct ReaderDecryptor<'a, T, C> where T: Read, C: Crypto {
    source: T,
    key: &'a Key<C>,
    nonce: Nonce<C>,
    buffer: Vec<u8>,
    chunk_size: usize,
    chunk_counter: usize,
}

impl<'a, T, C> ReaderDecryptor<'a, T, C> where T: Read, C: Crypto {
    pub fn new(key: &'a Key<C>, source: T) -> Self {
        Self {
            source,
            key,
            nonce: Default::default(),
            buffer: vec![],
            chunk_size: 0,
            chunk_counter: 0,
        }
    }
}

impl<'a, T, C> Read for ReaderDecryptor<'a, T, C> where T: Read, C: Crypto {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.chunk_size == 0 {
            let header = map_anyhow_io!(
                read_file_header::<C>(&mut self.source),
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
                core::decrypt_chunk::<C>(&buffer[..bytes_read].to_vec(),self.key,&self.nonce),
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

fn read_file_header<C>(source: &mut impl Read) -> Result<EncryptionFileHeader<C>> {
    //Read file version
    read_file_version(source)?;

    //Read file header
    let header = read_header(source)?;

    Ok(header)
}

fn read_file_version(source: &mut (impl Read + Sized)) -> Result<()> {
    let mut buffer = [0u8; 1];
    let n = source.read(&mut buffer)?;
    if buffer[0] != 1u8 || n != 1 {
        bail!("File version invalid")
    }
    Ok(())
}

fn read_header<C>(source: &mut (impl Read + Sized)) -> Result<EncryptionFileHeader<C>> {
    //Read context size
    let mut buffer_2 = [0u8; 2];
    let n = source.read(&mut buffer_2)?;
    if n != 2 {
        bail!("Error reading context size")
    }
    let context_size = u16::from_le_bytes(buffer_2);

    //Read context
    let mut buffer_context = vec![0; context_size as usize];
    let n = source.read(&mut buffer_context)?;
    if n != context_size as usize {
        bail!("Error reading context")
    }

    let file_header = map_anyhow_io!(
        serde_json::from_slice(buffer_context.as_slice()),
        "Error deserializing file header"
    )?;

    Ok(file_header)
}

fn read_chunk_header(source: &mut impl Read) -> Result<()> {
    let mut small_buffer = [0u8; 4];
    let size = source.read(&mut small_buffer)?;
    if size == 0 { return Ok(()); }
    if size == 4 && small_buffer.iter().all(|&x| x == 0u8)
    {
        Ok(())
    } else {
        bail!("Chunk header is not valid: {:?}", small_buffer);
    }
}