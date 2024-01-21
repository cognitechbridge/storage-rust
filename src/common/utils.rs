use std::fs::create_dir_all;
use std::path::PathBuf;
use regex::Regex;
use anyhow::{anyhow, Result};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;

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


pub fn get_cache_path() -> Result<PathBuf> {
    let mut path = dirs::cache_dir().ok_or(anyhow!("Cannot find cache folder"))?;
    path.push(".ctb");
    if !path.exists() {
        create_dir_all(&path)?;
    }
    Ok(path)
}



pub fn base64_decode(str: &str) -> Result<Vec<u8>> {
    let vec = BASE64_STANDARD.decode(str.trim())?;
    Ok(vec)
}
