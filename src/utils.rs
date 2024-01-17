pub mod as_array;

use std::fs::create_dir_all;
use std::path::PathBuf;
use regex::Regex;
use anyhow::{anyhow, bail, Result};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use generic_array::{ArrayLength, GenericArray};

pub fn type_name_of<T>() -> Result<String> {
    let full_name = std::any::type_name::<T>();
    let re = Regex::new(r"::([a-zA-Z0-9_]+)")?;
    let res = re.captures(full_name)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));
    res.ok_or(anyhow!("Error reading encryptor type string"))
}


pub fn get_user_path() -> Result<PathBuf> {
    let mut path = dirs::home_dir().ok_or(anyhow!("Cannot find user home"))?;
    path.push(".ctb");
    if !path.exists() {
        create_dir_all(&path)?;
    }
    Ok(path)
}

pub fn vec_to_generic_array<N>(vec: Vec<u8>) -> Result<GenericArray<u8, N>> where N: ArrayLength<u8> {
    if vec.len() != N::to_usize() {
        bail!("Decoded size doesn't match expected size.")
    }
    let mut arr: GenericArray<u8, N> = Default::default();
    arr.iter_mut().zip(vec).for_each(|(place, element)| *place = element);
    Ok(arr)
}

pub fn base64_decode(str: &str) -> Result<Vec<u8>> {
    let vec = BASE64_STANDARD.decode(str.trim().to_string())?;
    Ok(vec)
}
