extern crate rand;
extern crate chacha20;



use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use rand::Rng;
use hex_literal::hex;
// use chacha20::Key;
// use chacha20::Nonce;

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce, Key
};

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("Hello, world!");

    let files_bytes= read_file_to_vec("D:\\1.bin");
    let bytes = match files_bytes {
        Ok(val) => val,
        Err(e) => {
            panic!("Error occurred: {:?}", e)
        }
    };

    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    // let mut rng = rand::thread_rng();
    // let key = rng.gen::<[u8; 32]>();  // 32 bytes key
    // let nonce = rng.gen::<[u8; 12]>(); // 12 bytes nonce for ChaCha20

    //println!("{:x?}", &bytes);

    let cyphered = encrypt(bytes, &key, &nonce);
    //println!("{:x?}", cyphered);

    //let re = decrypt(cyphered, &key, &nonce);
    //println!("{:x?}", re);
}

fn read_file_to_vec(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn encrypt(plain:Vec<u8>, key:&Key, nonce:&Nonce) -> Vec<u8> {

    let mut cipher = ChaCha20Poly1305::new(&key);
    //let mut buffer = data.clone();
    let cipher_result = cipher.encrypt(&nonce, plain.as_ref());

    let value = match cipher_result {
        Ok(val) => val,
        Err(e) => {
            panic!("Error occurred: {:?}", e)
        }
    };
    return  value;
}

fn decrypt(encrypted:Vec<u8>, key:&Key, nonce:&Nonce) -> Vec<u8> {

    let mut cipher = ChaCha20Poly1305::new(&key);
    //let mut buffer = data.clone();
    let plain_result = cipher.decrypt(&nonce, encrypted.as_ref());

    let value = match plain_result {
        Ok(val) => val,
        Err(e) => {
            panic!("Error occurred: {:?}", e)
        }
    };
    return  value;
}


