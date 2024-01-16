use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};
use anyhow::{bail, Result};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use generic_array::ArrayLength;
use crate::keystore::{Key, KeyStore};

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableKeyStore {
    pub data_key_map: BTreeMap<String, String>,
}

impl<N: ArrayLength<u8>> KeyStore<N> {
    pub fn serialize(&self) -> Result<String> {
        let pair = self.data_key_map
            .iter()
            .map(|(k, v)| format!("{}:{}", k, BASE64_STANDARD.encode(v).to_string()))
            .collect::<Vec<String>>()
            .join("\n");
        return Ok(pair);
    }
    pub fn from_serialized(str: &String) -> Result<Self> {
        let mut map: HashMap<String, Key<N>> = HashMap::new();
        for line in str.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key_id = parts[0].trim().to_string();
                let key_vec = BASE64_STANDARD.decode(parts[1].trim().to_string())?;
                if key_vec.len() != N::to_usize() {
                    bail!("Decoded key size doesn't match store's.")
                }
                let mut key_arr: Key<N> = Default::default();
                key_arr.iter_mut().zip(key_vec).for_each(|(place, element)| *place = element);
                map.insert(key_id, key_arr);
            }
        }
        let store = KeyStore {
            data_key_map: map,
            ..Default::default()
        };
        return Ok(store);
    }
}