use generic_array::{ArrayLength, GenericArray};
use num_bigint::BigUint;
use num_traits::One;

pub fn increase_bytes_le<T>(nonce: &mut GenericArray<u8, T>) where T: ArrayLength<u8> {
    let mut number = BigUint::from_bytes_le(nonce);
    number += BigUint::one();
    let new_bytes = number.to_bytes_le();
    let min_len = std::cmp::min(nonce.len(), new_bytes.len());
    nonce[..min_len].copy_from_slice(&new_bytes[..min_len]);
}