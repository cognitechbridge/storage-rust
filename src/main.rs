mod encryptor;
mod s3_file_storage;
mod encryptor_file_generator;
mod stream_decryptor;


use chacha20poly1305::{
    aead::{AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

use std::fs::File;
use std::io::{Read, Write};
use encryptor::AsEncryptedIterator;
use crate::encryptor::{decrypt, Key, Nonce};
use crate::encryptor_file_generator::EncryptedFileGenerator;
use crate::stream_decryptor::reader_decrypt;


const CHUNK_SIZE: u64 = 1024 * 1024 * 5;

//#[tokio::main]
fn main() {
    println!("Hello, world!");

    // let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    // let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let mut key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let mut nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per

    for i in 0..key.len() {
        key[i] = i as u8;
    }
    for i in 0..nonce.len() {
        nonce[i] = i as u8;
    }

    // let file = File::create("D:\\File2.rtf").unwrap();
    // let mut writer = BufWriter::new(file);
    //
    // encryptor::process_encrypted_data(
    //     File::open("D:\\File.rtf").unwrap().to_encrypted_iterator(key, nonce, 100 * 1024),
    //     &mut writer, nonce,
    //     key);

    // let mut file = File::create("D:\\Sample.txt").expect("Could not create sample file.");
    // // Loop until the file is 5 chunks.
    // while file.metadata().unwrap().len() <= CHUNK_SIZE * 4 {
    //     let rand_string = "CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB CTB ";
    //     let return_string: String = "\n".to_string();
    //     file.write_all(rand_string.as_ref())
    //         .expect("Error writing to file.");
    //     file.write_all(return_string.as_ref())
    //         .expect("Error writing to file.");
    // }

    // let mut reader = File::open("D:\\Sample.txt")
    //     .unwrap()
    //     .to_encrypted_iterator(key, nonce, CHUNK_SIZE as usize)
    //     .to_encrypted_file_generator();
    // s3_file_storage::upload(&mut reader, "Hi3.txt".to_string()).await;

    // let mut file = File::create("D:\\DDD.txt").unwrap();
    // crate::s3_file_storage::download(&mut file,"Hi3.txt".to_string()).await;

    let mut file = File::open("D:\\DDD.txt").unwrap();
    reader_decrypt(&mut file, &key, &nonce);

    // let i = File::open("D:\\File.rtf").unwrap().to_encrypted_iterator(key, nonce, 100 * 1024);
    // let mut x = EncryptedFileGenerator::new(i);
    // let mut buffer = [0u8;110];
    // loop {
    //     match x.read(&mut buffer) {
    //         Ok(0) => break,
    //         Ok(n) => {
    //             println!("{:x?}", &buffer[..n])
    //         },
    //         Err(_e) => return, // Handle read error
    //     }
    // }
}

