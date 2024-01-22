use super::{
    KeyStore,
    Crypto, Key, Nonce,
};
use crate::common::{
    utils::base64_decode,
    GenericArrayFrom,
};
use aead::OsRng;
use anyhow::Result;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;

impl<C: Crypto> KeyStore<C> {
    pub fn serialize_key_pair(&self, key: Key<C>) -> Result<(String, String)> {
        let nonce = C::generate_nonce(OsRng);
        let cipher = C::new(&self.root_key);
        let ciphered = map_anyhow_io!(cipher.encrypt(&nonce, key.as_ref()), "Error encrypting data key")?;
        let res = (
            BASE64_STANDARD.encode(nonce),
            BASE64_STANDARD.encode(ciphered)
        );
        Ok(res)
    }
    pub fn deserialize_key_pair(&self, nonce: &str, ciphered: &str) -> Result<Key<C>> {
        let cipher = C::new(&self.root_key);

        let nonce_vec = base64_decode(nonce)?;
        let nonce = Nonce::<C>::try_from_vec(nonce_vec)?;
        let ciphered_vec = base64_decode(ciphered)?;
        let key_vec = map_anyhow_io!(cipher.decrypt(&nonce, ciphered_vec.as_ref()), "Error decrypting store key")?;
        let key = Key::<C>::try_from_vec(key_vec)?;
        Ok(key)
    }
}