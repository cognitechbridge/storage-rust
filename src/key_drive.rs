use aead::consts::U32;
use generic_array::GenericArray;
use hmac::{Hmac, Mac};
use sha3::Sha3_256;
use anyhow::Result;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

use argon2::{Algorithm, Argon2, Params, Version};

const MAX_ITERATION: u32 = 10000;

pub type Key = GenericArray<u8, U32>;

pub fn generate_key(root_key: &Key, context: &[u8]) -> Result<Key> {
    let mut derived_key = [0u8; 32];
    let params = Params::new(1024 * 19, 2, 1, None).unwrap();
    let x = Argon2::new(Algorithm::default(), Version::default(), params);
    x.hash_password_into(root_key, context, &mut derived_key);

    return Ok(Key::from(derived_key));
}