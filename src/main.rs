mod encryptor;
#[macro_use]
mod macros;
mod storage;
mod key_drive;


use chacha20poly1305::{
    aead::{KeyInit, OsRng},
    ChaCha20Poly1305,
};

use std::fs::File;
use encryptor::types::Crypto;
use std::io::{Read, Write};
use aead::AeadCore;
use uuid::{NoContext, Uuid};
use uuid::timestamp::Timestamp;
use crate::encryptor::{ToPlainStream, ToEncryptedStream};
use crate::key_drive::generate_key;
use crate::storage::*;


const CHUNK_SIZE: u64 = 1024 * 1024 * 5;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut key = Crypto::generate_key(&mut OsRng);
    println!("Key: {:x?}", key);

    for i in 0..key.len() {
        key[i] = i as u8;
    }
    let my_uuid = Uuid::new_v7(Timestamp::now(NoContext));
    let x = generate_key(&key, my_uuid.as_bytes()).unwrap();
    println!("Derived 1:{:x?}", x);

    let my_uuid = Uuid::new_v7(Timestamp::now(NoContext));
    let x = generate_key(&key, my_uuid.as_bytes()).unwrap();
    println!("Derived 2:{:x?}", x);

    // ************************ Generate Sample file *****************************

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

    // ***************** Storage ***********************************

    // let storage = crate::storage::s3::S3Storage::new("ctb-test-2".to_string(), 10 * 1024 * 1024);

    // ************************ Upload *****************************

    // let mut reader = File::open("D:\\Sample.txt")
    //     .unwrap()
    //     .to_encrypted_stream(&key, CHUNK_SIZE as usize);
    //
    // storage.upload(&mut reader, "Sample-Encrypted-2.txt".to_string()).await.unwrap();


    // ************************ Download *****************************

    // let download_file_path = "D:\\Sample-Encrypted.txt";
    // let decrypt_file_path = "D:\\Sample-UnEncrypted.txt";
    //
    // let mut file = File::create(download_file_path).unwrap();
    // storage.download(&mut file,"Sample-Encrypted.txt".to_string()).await.unwrap();
    //
    // let mut file = File::open(download_file_path).unwrap().to_plain_stream(&key);
    // let mut output_file = File::create(decrypt_file_path).unwrap();
    // let mut buffer = vec![0; 1024*1024*100];
    // loop {
    //     // Read up to 1KB from the input file
    //     let bytes_read = file.read(&mut buffer).unwrap();
    //
    //     // If no bytes were read, end of file is reached
    //     if bytes_read == 0 {
    //         break;
    //     }
    //
    //     // Write the bytes to the output file
    //     output_file.write_all(&buffer[..bytes_read]).unwrap();
    // }
}

