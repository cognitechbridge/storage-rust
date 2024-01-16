extern crate core;

mod encryptor;
#[macro_use]
mod macros;
mod storage;
mod keystore;
mod utils;
mod client_persistence;


use chacha20poly1305::{aead::{KeyInit, OsRng}, ChaCha20Poly1305 as Crypto, XChaCha20Poly1305};

use std::fs::File;
use std::io::{Read, Write};
use aead::AeadCore;
use crypto_common::KeySizeUser;
use uuid::{NoContext, Uuid};
use uuid::timestamp::Timestamp;
use crate::client_persistence::ClientPersistence;
use crate::client_persistence::user_file::ClientFolderPersistence;
use crate::encryptor::{ToPlainStream, ToEncryptedStream, EncryptionFileHeader};
use crate::keystore::{DataKeyRecoveryGenerator, KeyStore};
use crate::storage::*;

type KeySize = <Crypto as KeySizeUser>::KeySize;

const CHUNK_SIZE: u64 = 1024 * 1024 * 5;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut key = Crypto::generate_key(&mut OsRng);

    for i in 0..key.len() {
        key[i] = 0 as u8;
    }

    let uuid = Uuid::new_v7(Timestamp::now(NoContext));

    let mut store: KeyStore<KeySize, XChaCha20Poly1305> = KeyStore::new(key);
    store.load_from_persist().unwrap();

    let data_key_pair = store.generate_key_pair(&uuid, OsRng).unwrap();
    let blob = data_key_pair.recovery_blob.to_string();
    let data_key = data_key_pair.key;

    //let t = ClientFolderPersistence {};
    //ClientFolderPersistence::load_client_config("Test".to_string());

    //println!("{:?}", data_key);
    let s = store.serialize_store().unwrap();
    println!("{}", s);


    let mut store_2: KeyStore<KeySize, XChaCha20Poly1305> = KeyStore::new(key);
    store_2.load_from_string(&s).unwrap();
    println!("{}", store_2.serialize_store().unwrap());




    // let x = type_name_of::<ChaCha20Poly1305>().unwrap();
    // println!("{}", x);

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

    let storage = s3::S3Storage::new("ctb-test-2".to_string(), 10 * 1024 * 1024);

    // ************************ Upload *****************************

    let header = EncryptionFileHeader {
        client_id: "client-id".to_string(),
        file_id: uuid.to_string(),
        recovery: blob,
        chunk_size: 10,
        ..Default::default()
    };
    let mut reader = File::open("D:\\Sample.txt")
        .unwrap()
        .to_encrypted_stream::<Crypto>(&data_key, header).unwrap();


    // let mut output_file = File::create("D:\\Test.txt").unwrap();
    // let mut buffer = vec![0; 1024 * 1024 * 100];
    // loop {
    //     // Read up to 1KB from the input file
    //     let bytes_read = reader.read(&mut buffer).unwrap();
    //
    //     // If no bytes were read, end of file is reached
    //     if bytes_read == 0 {
    //         break;
    //     }
    //
    //     // Write the bytes to the output file
    //     output_file.write_all(&buffer[..bytes_read]).unwrap();
    // }


    storage.upload(&mut reader, uuid.to_string()).await.unwrap();


    // ************************ Download *****************************

    let download_file_path = "D:\\Sample-Encrypted.txt";
    let decrypt_file_path = "D:\\Sample-UnEncrypted.txt";
    //
    let mut file = File::create(download_file_path).unwrap();
    storage.download(&mut file, uuid.to_string()).await.unwrap();

    let mut file = File::open(download_file_path).unwrap().to_plain_stream::<Crypto>(&data_key);
    let mut output_file = File::create(decrypt_file_path).unwrap();
    let mut buffer = vec![0; 1024 * 1024 * 100];
    loop {
        // Read up to 1KB from the input file
        let bytes_read = file.read(&mut buffer).unwrap();

        // If no bytes were read, end of file is reached
        if bytes_read == 0 {
            break;
        }

        // Write the bytes to the output file
        output_file.write_all(&buffer[..bytes_read]).unwrap();
    }
}

