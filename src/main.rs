mod encryptor;


use chacha20poly1305::{
    aead::{AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305
};

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("Hello, world!");

    let bytes= read_file_to_vec("D:\\File.rtf").unwrap();


    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let mut nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    println!("{:x?}", &nonce);
    encryptor::increase_bytes_le(&mut nonce);
    println!("{:x?}", &nonce);

    let enc = encryptor::EncryptedChunker::new(
        File::open("D:\\File.rtf").unwrap(),
        key,
        nonce
    );


    let mut cyphered = encryptor::encrypt(bytes, &key, &nonce).unwrap();
    let re = encryptor::decrypt(cyphered, &key, &nonce).unwrap();
}

fn read_file_to_vec(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}



