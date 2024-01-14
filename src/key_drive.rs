use aead::consts::U32;
use generic_array::GenericArray;
use hmac::{Hmac, Mac};
use sha3::Sha3_256;
use anyhow::Result;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

const MAX_ITERATION: u32 = 10000;

pub type Key = GenericArray<u8, U32>;

pub fn generate_key(root_key: &Key, context: &[u8]) -> Result<Key> {
    let mut mac = Hmac::<Sha3_256>::new_from_slice(&root_key)?;
    let iteration = BigUint::from_bytes_le(root_key) % MAX_ITERATION;
    for _i in 0..iteration.to_u32().unwrap() {
        mac.update(root_key);
        mac.update(context);
    }
    let derived_key = mac.finalize().into_bytes();
    return Ok(derived_key);
}