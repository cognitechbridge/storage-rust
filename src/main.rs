mod encryptor;


use chacha20poly1305::{
    aead::{AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};

use std::fs::File;
use std::io::{BufWriter};
use encryptor::AsEncryptedIterator;

fn main() {
    println!("Hello, world!");

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let file = File::create("D:\\File2.rtf").unwrap();
    let mut writer = BufWriter::new(file);

    encryptor::process_encrypted_data(
        File::open("D:\\File.rtf").unwrap().to_encrypted_iterator(key, nonce, 100 * 1024),
        &mut writer, nonce,
        key);

    // println!("{:x?}", &buffer);
}


