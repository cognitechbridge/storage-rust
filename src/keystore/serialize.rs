use super::{
    Crypto
};
use aead::OsRng;
use anyhow::{anyhow, bail, Result};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use crate::keystore::{Key, KeyStore};
use crate::utils::{base64_decode, vec_to_generic_array};

const RECOVERY_KEY: &str = "---------------RECOVERY-------------";

impl<C: Crypto> KeyStore<C> {
    pub fn serialize_key_pair(&self, key_id: &str, key: &Key<C>) -> Result<String> {
        let nonce = C::generate_nonce(OsRng);
        let cipher = C::new(&self.root_key);
        let ciphered = map_anyhow_io!(cipher.encrypt(&nonce, key.as_ref()), "Error encrypting data key")?;
        let text = format!("{}:{}:{}", key_id, BASE64_STANDARD.encode(nonce).to_string(), BASE64_STANDARD.encode(ciphered).to_string());
        Ok(text)
    }
    pub fn deserialize_key_pair(&self, str: &str) -> Result<(String, Key<C>)> {
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
        Ok((key_id, key))
    }
    pub fn serialize_recovery_key(&self) -> Result<String> {
        let recovery_key = self.get_recovery_key().ok_or(anyhow!("Cannot find recovery key"))?;
        let res = self.serialize_key_pair(RECOVERY_KEY, &recovery_key)?;
        Ok(res)
    }
    pub fn serialize_store(&self) -> Result<String> {
        let recovery_str = self.serialize_recovery_key()?;
        let pairs_str = self.data_key_map
            .iter()
            .map(|(k, v)| self.serialize_key_pair(k, v))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n");
        let result = format!("{}\n{}", recovery_str, pairs_str);
        Ok(result)
    }
    pub fn load_from_string(&mut self, str: &str) -> Result<()> {
        for line in str.lines() {
            let (id, key) = self.deserialize_key_pair(line).unwrap();
            if id == RECOVERY_KEY {
                self.recovery_key = Some(key);
            } else {
                self.data_key_map.insert(id, key);
            }
        }
        self.loaded = true;
        Ok(())
    }
}