extern crate rand;
extern crate chacha20;



use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use rand::Rng;
use hex_literal::hex;
use chacha20::Key;
use chacha20::Nonce;

fn main() {
    println!("Hello, world!");

    let plaintext = "Hello";

    let mut rng = rand::thread_rng();
    let key = rng.gen::<[u8; 32]>();  // 32 bytes key
    let nonce = rng.gen::<[u8; 12]>(); // 12 bytes nonce for ChaCha20

    let my_string = "Hello, world!";
    let bytes = my_string.as_bytes().to_vec();
    println!("{:x?}", &bytes);

    let cyphered = encrypt(&bytes, &key.into(), &nonce.into());
    println!("{:x?}", cyphered);

    let re = encrypt(&cyphered, &key.into(), &nonce.into());
    println!("{:x?}", re);


}

fn encrypt(data:&Vec<u8>, key:&Key, nonce:&Nonce) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    //let nonce = rng.gen::<[u8; 12]>(); // 12 bytes nonce for ChaCha2

    let mut cipher = ChaCha20::new(&key, &nonce);
    let mut buffer = data.clone();
    cipher.apply_keystream(&mut buffer);

    return buffer.to_vec();
}

