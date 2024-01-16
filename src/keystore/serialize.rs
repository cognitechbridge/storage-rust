use aead::{Aead, OsRng};
use anyhow::{bail, Result};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use crypto_common::{KeyInit, KeySizeUser};
use generic_array::ArrayLength;
use crate::keystore::{Key, KeyStore};
use crate::utils::{base64_decode, vec_to_generic_array};

impl<N: ArrayLength<u8>, C: KeySizeUser<KeySize=N> + KeyInit + Aead> KeyStore<N, C> {
    pub fn serialize_key_pair(&self, key_id: &str, key: &Key<N>) -> Result<String> {
        let nonce = C::generate_nonce(OsRng);
        let cipher = C::new(&self.root_key);
        let ciphered = map_anyhow_io!(cipher.encrypt(&nonce, key.as_ref()), "Error encrypting data key")?;
        let text = format!("{}:{}:{}", key_id, BASE64_STANDARD.encode(nonce).to_string(), BASE64_STANDARD.encode(ciphered).to_string());
        return Ok(text);
    }
    pub fn deserialize_key_pair(&self, str: &str) -> Result<(String, Key<N>)> {
        let cipher = C::new(&self.root_key);
        let parts: Vec<&str> = str.splitn(3, ':').collect();
        if parts.len() != 3 {
            bail!("Parts not equal to 3")
        }
        let key_id = parts[0].trim().to_string();
        let nonce_vec = base64_decode(parts[1])?;
        let nonce = vec_to_generic_array(nonce_vec)?;
        let ciphered_vec = base64_decode(parts[2])?;
        let key_vec = map_anyhow_io!(cipher.decrypt(&nonce, ciphered_vec.as_ref()), "Error decrypting store key")?;
        let key = vec_to_generic_array(key_vec)?;
        return Ok((key_id, key));
    }
    pub fn serialize_store(&self) -> Result<String> {
        let pairs: Result<Vec<String>, _> = self.data_key_map
            .iter()
            .map(|(k, v)| self.serialize_key_pair(k, v))
            .collect();
        let result = pairs?.join("\n");
        return Ok(result);
    }
    pub fn load_from_string(&mut self, str: &str) -> Result<()> {
        for line in str.lines() {
            let (id, key) = self.deserialize_key_pair(line).unwrap();
            self.data_key_map.insert(id, key);
        }
        return Ok(());
    }
}