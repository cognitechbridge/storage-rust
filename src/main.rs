extern crate core;

mod encryptor;
#[macro_use]
mod macros;
mod storage;
mod keystore;
mod client_persistence;
mod common;


use crate::common::{
    utils::get_cache_path
};

use chacha20poly1305::{aead::{KeyInit, OsRng}, ChaCha20Poly1305 as Crypto, ChaCha20Poly1305, XChaCha20Poly1305};

use std::fs::{File, remove_file};
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use crypto_common::KeySizeUser;
use uuid::{NoContext, Uuid};
use uuid::timestamp::Timestamp;
use anyhow::Result;
use crate::client_persistence::ClientPersistence;
use crate::encryptor::{Decryptor, Encryptor};
use crate::keystore::{KeyStore};
use crate::storage::*;

type KeySize = <Crypto as KeySizeUser>::KeySize;

const CHUNK_SIZE: u64 = 1024 * 1024 * 5;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // let mut path = get_user_path().unwrap();
    // path.push("key_store_sqlite");
    // let mut conn = Connection::open(path).unwrap();
    //
    // let migrations = Migrations::new(vec![
    //     M::up(
    //         "CREATE TABLE keystore(id TEXT PRIMARY KEY, nonce TEXT NULL, key TEXT NOT NULL);"
    //     ),
    // ]);
    //
    // conn.pragma_update(None, "journal_mode", &"WAL").unwrap();
    // migrations.to_latest(&mut conn).unwrap();


    let mut key = Crypto::generate_key(&mut OsRng);

    for i in 0..key.len() {
        key[i] = 0 as u8;
    }

    let storage = s3::S3Storage::new(String::from("ctb-test-2"), 10 * 1024 * 1024);

    let mut store = Box::new(KeyStore::new(key));
    store.init().expect("Cannot init store");
    if store.get_recovery_key().is_none() {
        store.set_recover_key(key).expect("Cannot set recovery key");
    }
    //store.load_from_persist().unwrap();


    let file_id = safe_store_file("D:\\Sample.txt", &mut store, &storage)
        .await.expect("Could not upload the file");

    download("D:\\Sample-2.txt", file_id, &mut store, &storage)
        .await.expect("Could not download file");


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


    // ************************ Download *****************************

    // let download_file_path = "D:\\Sample-Encrypted.txt";
    // let decrypt_file_path = "D:\\Sample-UnEncrypted.txt";
    // //
    // let mut file = File::create(download_file_path).unwrap();
    // storage.download(&mut file, uuid.to_string()).await.unwrap();
    //
    // let mut file = File::open(download_file_path).unwrap().to_plain_stream::<Crypto>(&data_key);
    // let mut output_file = File::create(decrypt_file_path).unwrap();
    // let mut buffer = vec![0; 1024 * 1024 * 100];
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

pub async fn download(
    path: impl AsRef<Path>,
    file_id: String,
    store: &mut Box<KeyStore<XChaCha20Poly1305>>,
    storage: &impl StorageDownload,
) -> Result<()> {
    let mut cache_file_path = get_cache_path()?;
    cache_file_path.push(file_id.clone());

    let mut file = File::create(cache_file_path.clone()).unwrap();
    storage.download(&mut file, file_id.clone()).await?;

    let data_key = store.get(&file_id).expect("Key not found").expect("Key not found");

    let file_dc = File::open(cache_file_path.clone()).expect("Can not open the file");
    let mut decryptor = Decryptor::<Crypto>::new();
    let mut file = decryptor.decrypt(&data_key, file_dc).expect("Could not decrypt the file.");
    let mut output_file = File::create(path).unwrap();
    io::copy(&mut file, &mut output_file)?;
    remove_file(cache_file_path)?;
    Ok(())
}

pub async fn safe_store_file(
    path: impl AsRef<Path>,
    store: &mut Box<KeyStore<XChaCha20Poly1305>>,
    storage: &impl StorageUpload) -> Result<String> {
    let file = File::open(path).expect("Could not open file");

    let encryptor = Encryptor::<ChaCha20Poly1305>::new(
        String::from("client-id"),
        CHUNK_SIZE,
    );

    let uuid = Uuid::new_v7(Timestamp::now(NoContext));
    let data_key_pair = store.generate_key_pair(&uuid, OsRng).unwrap();
    let blob = data_key_pair.recovery_blob.to_string();
    let data_key = data_key_pair.key;

    let mut read = encryptor
        .encrypt(file, uuid.to_string(), &data_key, blob)?;

    storage.upload(&mut read, uuid.to_string()).await?;

    return Ok(uuid.to_string());
}
