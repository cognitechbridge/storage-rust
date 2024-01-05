mod encryptor;


use chacha20poly1305::{
    aead::{AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305
};

use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

fn main() {
    println!("Hello, world!");

    let bytes= read_file_to_vec("D:\\File.rtf").unwrap();


    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let mut nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let enc = encryptor::EncryptedChunker::new(
        File::open("D:\\File.rtf").unwrap(),
        key,
        nonce
    );

    let file = File::create("D:\\File2.rtf").unwrap();
    let mut writer = BufWriter::new(file);

    encryptor::process_encrypted_data(enc.map(|x| x.unwrap()), &mut writer, nonce, key);
}

fn read_file_to_vec(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}



