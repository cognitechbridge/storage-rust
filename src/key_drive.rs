use aead::consts::U32;
use generic_array::GenericArray;
use hmac::{Hmac, Mac};
use sha3::Sha3_256;
use anyhow::Result;


pub type Key = GenericArray<u8, U32>;

pub fn generate_key(root_key: &Key, context: &[u8]) -> Result<Key> {
    let mut mac = Hmac::<Sha3_256>::new_from_slice(&root_key)?;
    let itr = context.last().unwrap().clone();
    for i in 0..itr {
        mac.update(context);
    }
    let derived_key = mac.finalize().into_bytes();
    return Ok(derived_key);
}