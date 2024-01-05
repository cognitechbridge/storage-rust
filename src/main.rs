mod encryptor;
mod s3_file_storage;


use chacha20poly1305::{
    aead::{AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

use std::fs::File;
use std::io::Write;
use encryptor::AsEncryptedIterator;


const CHUNK_SIZE: u64 = 1024 * 1024 * 5;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

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

    let iterator = File::open("D:\\Sample.txt")
        .unwrap()
        .to_encrypted_iterator(key, nonce, CHUNK_SIZE as usize);
    s3_file_storage::upload(iterator, "Hi2.txt".to_string()).await;

    // println!("{:x?}", &buffer);
}

