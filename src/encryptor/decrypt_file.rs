use std::io::Read;
use anyhow::{anyhow, bail};
use num_bigint::{BigUint};
use num_traits::ToPrimitive;
use crate::map_anyhow_io;
use super::{utils, core, ToPlainStream};
use super::types::{Result, Key, Nonce};

pub struct ReaderDecryptor<T> where T: Read {
    source: T,
    key: Key,
    nonce: Nonce,
    buffer: Vec<u8>,
    chunk_size: usize,
    chunk_counter: usize,
}

impl<T: Read> ToPlainStream<ReaderDecryptor<T>> for T {
    fn to_plain_stream(self, key: Key) -> ReaderDecryptor<T> {
        return ReaderDecryptor {
            source: self,
            key,
            nonce: Nonce::default(),
            buffer: vec![],
            chunk_size: 0,
            chunk_counter: 0,
        };
    }
}

impl<T> Read for ReaderDecryptor<T> where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.chunk_size == 0 {
            self.chunk_size = map_anyhow_io!(
                read_chunk_size(&mut self.source),
                "Error reading chunk size"
            )?;
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
                core::decrypt_chunk(&buffer[..bytes_read].to_vec(),&self.key,&self.nonce),
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

fn read_chunk_size<T>(source: &mut T) -> Result<usize> where T: Read {
    let mut small_buffer = [0u8; 4];
    source.read(&mut small_buffer)?;
    let res = BigUint::from_bytes_le(&mut small_buffer)
        .to_u64()
        .map(|x| x as usize)
        .ok_or(anyhow!("Chunk size conversion error"));
    return res;
}

fn read_chunk_header<T>(source: &mut T) -> Result<()> where T: Read {
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