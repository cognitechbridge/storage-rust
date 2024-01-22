use anyhow::bail;
use rusqlite::{Error, params};
use crate::keystore::SerializedKey;
use crate::persistence::SqlLiteConnection;

impl crate::keystore::KeyStorePersist for SqlLiteConnection {
    fn save_key(&self, serialized_key: SerializedKey) -> anyhow::Result<()> {
        self.db_conn.execute(
            "INSERT INTO keystore (id, nonce, key, tag) VALUES (?1, ?2, ?3, ?4)",
            params![serialized_key.id, serialized_key.nonce, serialized_key.key, serialized_key.tag],
        )?;
        Ok(())
    }

    fn get_key(&self, key_id: &str) -> anyhow::Result<Option<SerializedKey>> {
        match self.db_conn.query_row(
            "SELECT id, nonce, key, tag FROM keystore WHERE id = ?1",
            params![key_id],
            |row| Ok(SerializedKey {
                id: row.get(0)?,
                nonce: row.get(1)?,
                key: row.get(2)?,
                tag: row.get(3)?,
            }),
        ) {
            Ok(sk) => Ok(Some(sk)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => bail!("Query failed: {}", e),
        }
    }

    fn get_with_tag(&self, tag: &str) -> anyhow::Result<Option<SerializedKey>> {
        match self.db_conn.query_row(
            "SELECT id, nonce, key, tag FROM keystore WHERE tag = ?1",
            params![tag],
            |row| Ok(SerializedKey {
                id: row.get(0)?,
                nonce: row.get(1)?,
                key: row.get(2)?,
                tag: row.get(3)?,
            }),
        ) {
            Ok(sk) => Ok(Some(sk)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => bail!("Query failed: {}", e),
        }
    }
}



