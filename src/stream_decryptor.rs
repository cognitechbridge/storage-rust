use std::fs::File;
use std::io::{Read, Write};
use num_bigint::{BigUint};
use num_traits::ToPrimitive;
use log::error;
use crate::encryptor;
use crate::encryptor::{Key, Nonce};

pub fn reader_decrypt<T>(reader: &mut T, key: &Key, nonce: &Nonce) where T: Read {
    let mut writer = File::create("D:\\DC.txt").unwrap();

    let mut n = nonce.clone();

    let mut small_buffer = [0u8;4];
    reader.read(&mut small_buffer).unwrap();
    let ch_size = BigUint::from_bytes_le(&mut small_buffer).to_u64().unwrap() as usize;
    let mut buffer = vec![0u8; ch_size];
    loop {
        reader.read(&mut small_buffer).unwrap();
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(size) => size,
            Err(e) => 0,
        };
        let decrypted_data = encryptor::decrypt(&buffer[..bytes_read].to_vec(), key, &n).unwrap();
        encryptor::increase_bytes_le(&mut n);
        writer.write_all(&decrypted_data).unwrap();
    }

}