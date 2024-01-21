mod serialize;
mod recovery;
mod persistence;

use anyhow::Result;
use std::marker::PhantomData;
use std::sync::Arc;
use crate::common::{Crypto, Key, Nonce};
pub use persistence::SerializedPersistKeyStore;

pub struct PersistKeyStore<C: Crypto> {
    root_key: Key<C>,
    persist: Arc<dyn SerializedPersistKeyStore>,
    alg: PhantomData<C>,
}


impl<C: Crypto> PersistKeyStore<C> {
    pub fn new(root_key: Key<C>, persist: Arc<dyn SerializedPersistKeyStore>) -> Self <> {
        PersistKeyStore {
            root_key,
            persist,
            alg: Default::default(),
        }
    }

    fn persist_key(&self, key_id: &str, key: Key<C>, tag: &str) -> Result<()> {
        let (nonce_hashed, key_hashed) = self.serialize_key_pair(key)?;
        self.persist.save_key(&key_id, &nonce_hashed, &key_hashed, tag)?;
        Ok(())
    }

    pub fn insert(&mut self, key_id: &str, key: Key<C>) -> Result<()> {
        self.persist_key(key_id, key, "DK")?;
        Ok(())
    }

    pub fn get(&self, key_id: &str) -> Result<Option<Key<C>>> {
        let res = self.persist.get_key(key_id)?;
        match res {
            None => Ok(None),
            Some((nonce, key)) => {
                let key = self.deserialize_key_pair(&nonce, &key)?.clone();
                Ok(Some(key))
            }
        }
    }

    pub fn get_with_tag(&self, tag: &str) -> Result<Option<(String, Key<C>)>> {
        let res = self.persist.get_with_tag(tag)?;
        match res {
            None => Ok(None),
            Some((id, nonce, key)) => {
                let key = self.deserialize_key_pair(&nonce, &key)?.clone();
                Ok(Some((id, key)))
            }
        }
    }
}
