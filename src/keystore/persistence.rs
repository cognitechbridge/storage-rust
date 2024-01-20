use anyhow::{bail, Result};
use rusqlite::{Connection, Error, params};
use crate::common::{
    utils::get_user_path,
    Crypto, Key,
};
use crate::keystore::KeyStore;

pub struct KeyStorePersist {
    pub db_conn: Connection,
}

mod migrations;

impl KeyStorePersist {
    pub fn new() -> Result<Self> {
        let mut path = get_user_path()?;
        path.push("db.db3");
        let conn = Connection::open(path).unwrap();
        Ok(Self {
            db_conn: conn
        })
    }

    pub fn init(&mut self) -> Result<()>{
        let migrations = migrations::get_migrations();
        self.db_conn.pragma_update(None, "journal_mode", &"WAL").unwrap();
        migrations.to_latest(&mut self.db_conn)?;
        Ok(())
    }

    pub fn persist_key(&self, key_id: &str, nonce: &str, key: &str) -> Result<()> {
        self.db_conn.execute(
            "INSERT INTO keystore (id, nonce, key) VALUES (?1, ?2, ?3)",
            params![key_id, nonce, key],
        )?;
        Ok(())
    }

    pub fn get_key(&self, key_id: &str) -> Result<Option<(String, String)>> {
        match self.db_conn.query_row(
            "SELECT id, nonce, key FROM keystore WHERE id = ?1",
            params![key_id],
            |row| Ok((row.get(1)?, row.get(2)?)),
        ) {
            Ok((nonce, key)) => Ok(Some((nonce, key))),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => bail!("Query failed: {}", e),
        }
    }
}

impl<C: Crypto> KeyStore<C> {
    pub fn persist_key(&self, key_id: &str, key: Key<C>) -> Result<()> {
        let (nonce_hashed, key_hashed) = self.serialize_key_pair(key)?;
        self.persist.persist_key(&key_id, &nonce_hashed, &key_hashed)?;
        Ok(())
    }
}