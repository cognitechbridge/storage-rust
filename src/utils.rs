use regex::Regex;
use anyhow::{anyhow, Result};

pub fn type_name_of<T>() -> Result<String> {
    let full_name = std::any::type_name::<T>();
    let re = Regex::new(r"::([a-zA-Z0-9_]+)")?;
    let res = re.captures(full_name)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));
    return res.ok_or(anyhow!("Error reading encryptor type string"));
}
