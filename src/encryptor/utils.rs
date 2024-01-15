use generic_array::{ArrayLength, GenericArray};
use num_bigint::BigUint;
use num_traits::One;
use regex::Regex;

pub fn increase_bytes_le<T>(nonce: &mut GenericArray<u8, T>) where T: ArrayLength<u8> {
    let mut number = BigUint::from_bytes_le(nonce);
    number += BigUint::one();
    let new_bytes = number.to_bytes_le();
    let min_len = std::cmp::min(nonce.len(), new_bytes.len());
    nonce[..min_len].copy_from_slice(&new_bytes[..min_len]);
}

pub fn type_name_of<T>() -> String {
    let full_name = std::any::type_name::<T>();
    let re = Regex::new(r"::([a-zA-Z0-9_]+)").unwrap();
    let res = re.captures(full_name)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));
    return res.unwrap_or_default();
}