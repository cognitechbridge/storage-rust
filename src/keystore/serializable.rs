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
    pub fn serialize(&mut self) -> Result<String> {
        let pair = self.data_key_map
            .iter()
            .map(|(k, v)| (k, BASE64_STANDARD.encode(v).to_string()));
        let mut map = BTreeMap::new();
        for (key, value) in pair {
            map.insert(key.to_string(), value);
        }
        let x = SerializableKeyStore {
            data_key_map: map
        };
        let se = serde_json::to_string(&x)?;
        return Ok(se);
    }
    pub fn from_serialized(str: &String) -> Result<Self> {
        let ser_store: SerializableKeyStore = serde_json::from_str(&str)?;
        let mut map: HashMap<String, Key<N>> = HashMap::new();
        for (key_id, key_encoded) in ser_store.data_key_map {
            let key_vec = BASE64_STANDARD.decode(key_encoded)?;
            if key_vec.len() != N::to_usize() {
                bail!("Decoded key size doesn't match store's.")
            }
            let mut key_arr: Key<N> = Default::default();
            key_arr.iter_mut().zip(key_vec).for_each(|(place, element)| *place = element);
            map.insert(key_id, key_arr);
        }
        let store = KeyStore {
            data_key_map: map,
            ..Default::default()
        };
        return Ok(store);
    }
}