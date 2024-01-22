mod serialize;
mod recovery;

use anyhow::Result;
use std::marker::PhantomData;
use std::sync::Arc;
use crate::common::{Crypto, Key, Nonce};

pub struct KeyStore<C: Crypto> {
    root_key: Key<C>,
    persist: Arc<dyn KeyStorePersist>,
    alg: PhantomData<C>,
}


impl<C: Crypto> KeyStore<C> {
    pub fn new(root_key: Key<C>, persist: Arc<dyn KeyStorePersist>) -> Self <> {
        KeyStore {
            root_key,
            persist,
            alg: Default::default(),
        }
    }

    fn persist_key(&self, key_id: &str, key: &Key<C>, tag: &str) -> Result<()> {
        let (nonce_hashed, key_hashed) = self.serialize_key_pair(key)?;
        let sk = SerializedKey {
            id: String::from(key_id),
            nonce: nonce_hashed,
            key: key_hashed,
            tag: String::from(tag),
        };
        self.persist.save_key(sk)?;
        Ok(())
    }

    pub fn insert(&mut self, key_id: &str, key: &Key<C>) -> Result<()> {
        self.persist_key(key_id, key, "DK")?;
        Ok(())
    }

    pub fn get(&self, key_id: &str) -> Result<Option<Key<C>>> {
        let res = self.persist.get_key(key_id)?;
        match res {
            None => Ok(None),
            Some(sk) => {
                let key = self.deserialize_key_pair(&sk.nonce, &sk.key)?.clone();
                Ok(Some(key))
            }
        }
    }

    pub fn get_with_tag(&self, tag: &str) -> Result<Option<(String, Key<C>)>> {
        let res = self.persist.get_with_tag(tag)?;
        match res {
            None => Ok(None),
            Some(sk) => {
                let key = self.deserialize_key_pair(&sk.nonce, &sk.key)?.clone();
                Ok(Some((sk.id, key)))
            }
        }
    }
}

pub struct SerializedKey {
    pub id: String,
    pub nonce: String,
    pub key: String,
    pub tag: String,
}

pub trait KeyStorePersist {
    fn save_key(&self, serialized_key: SerializedKey) -> Result<()>;
    fn get_key(&self, key_id: &str) -> Result<Option<SerializedKey>>;
    fn get_with_tag(&self, tag: &str) -> Result<Option<SerializedKey>>;
}
