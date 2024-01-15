mod hash_map_serde;

use std::collections::HashMap;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use chacha20poly1305::ChaCha20Poly1305;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;
use crate::encryptor::TKey;


#[derive(Serialize, Deserialize, Debug)]
pub struct KeyStore {
    #[serde(with = "hash_map_serde")]
    pub x: HashMap<String, TKey<ChaCha20Poly1305>>,
}


impl KeyStore {
    pub fn insert(&mut self, key: String, value: TKey<ChaCha20Poly1305>) {
        self.x.insert(key, value);
    }
}
