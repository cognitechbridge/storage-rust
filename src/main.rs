extern crate core;

mod encryptor;
#[macro_use]
mod macros;
mod storage;
mod keystore;mod common;
mod file_system;
mod persistence;


use crate::common::{
    utils::get_cache_path
};

use chacha20poly1305::{aead::{KeyInit, OsRng}, ChaCha20Poly1305 as Crypto, ChaCha20Poly1305, XChaCha20Poly1305};

use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use crypto_common::KeySizeUser;
use uuid::{NoContext, Uuid};
use uuid::timestamp::Timestamp;
use anyhow::Result;
use tempfile::NamedTempFile;
use crate::encryptor::{Decryptor, Encryptor};
use crate::file_system::PersistFileSystem;
use crate::keystore::{PersistKeyStore, SerializedPersistKeyStore};
use crate::persistence::SqlLiteConnection;
use crate::storage::*;

type KeyStore = PersistKeyStore<XChaCha20Poly1305>;

const CHUNK_SIZE: u64 = 1024 * 1024 * 5;

#[tokio::main]
async fn main() {
    println!("Hello, world!");


    let mut key = Crypto::generate_key(&mut OsRng);

    for i in 0..key.len() {
        key[i] = 0 as u8;
    }

    let storage = s3::S3Storage::new("ctb-test-2", 10 * 1024 * 1024);

    //Create sqlite connection
    let mut sql_connection = SqlLiteConnection::new().expect("Cannot create sql");
    sql_connection.init().expect("Cannot init sql");
    let sql = Arc::new(sql_connection);

    //Create file store
    let file_store = Box::new(PersistFileSystem::new(sql.clone()));

    //Create key store
    let mut key_store = Box::new(PersistKeyStore::new(key, sql));
    if key_store.get_recovery_key().is_none() {
        let uuid = Uuid::new_v7(Timestamp::now(NoContext));
        key_store.set_recover_key(&uuid.to_string(), key).expect("Cannot set recovery key");
    }

    let friendly_path = "Sample.txt";

    safe_store_file("D:\\Sample.txt", friendly_path, &mut key_store, &storage, &file_store)
        .await.expect("Could not upload the file");


    download("D:\\Sample-2.txt", friendly_path, &key_store, &storage, &file_store)
        .await.expect("Could not download file");
}

pub async fn download
(
    path: impl AsRef<Path>,
    friendly_path: &str,
    store: &KeyStore,
    storage: &impl StorageDownload,
    file_store: &PersistFileSystem,
)
    -> Result<()>
{
    //Get file id
    let file_id = file_store.get_path(&friendly_path)?.expect("File not found");

    //Get data key
    let data_key = store.get(&file_id).expect("Key not found").expect("Key not found");

    //Create temp file
    let mut temp_file = NamedTempFile::new()?;

    //Download to temp file
    storage.download(&mut temp_file, &file_id).await?;

    //Decrypt the file
    let mut decryptor = Decryptor::<Crypto>::new();
    let mut file = decryptor.decrypt(&data_key, temp_file.reopen()?).expect("Could not decrypt the file.");

    //Write to output file
    let mut output_file = File::create(path).unwrap();
    io::copy(&mut file, &mut output_file)?;

    Ok(())
}

pub async fn safe_store_file
(
    path: impl AsRef<Path>,
    friendly_path: &str,
    store: &mut KeyStore,
    storage: &impl StorageUpload,
    file_store: &PersistFileSystem,
)
    -> Result<String>
{
    //Open file
    let file = File::open(path).expect("Could not open file");

    //Create id
    let uuid = Uuid::new_v7(Timestamp::now(NoContext));

    //Create data key
    let data_key_pair = store.generate_key_pair(&uuid, OsRng).unwrap();

    //Crate encrypt and encrypt
    let encryptor = Encryptor::<ChaCha20Poly1305>::new(
        "client-id",
        CHUNK_SIZE,
    );
    let mut read = encryptor
        .encrypt(file, &uuid.to_string(), &data_key_pair.key, &data_key_pair.recovery_blob)?;

    //Upload
    storage.upload(&mut read, &uuid.to_string()).await?;

    //Save file path
    file_store.save_path(&uuid.to_string(), friendly_path)?;

    return Ok(uuid.to_string());
}
