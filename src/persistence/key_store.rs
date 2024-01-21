use anyhow::bail;
use rusqlite::{Error, params};
use crate::persistence::SqlLiteConnection;

impl crate::keystore::SerializedPersistKeyStore for SqlLiteConnection {

    fn save_key(&self, id: &str, nonce: &str, key: &str, tag: &str) -> anyhow::Result<()> {
        self.db_conn.execute(
            "INSERT INTO keystore (id, nonce, key, tag) VALUES (?1, ?2, ?3, ?4)",
            params![id, nonce, key, tag],
        )?;
        Ok(())
    }

    fn get_key(&self, key_id: &str) -> anyhow::Result<Option<(String, String)>> {
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

    fn get_with_tag(&self, tag: &str) -> anyhow::Result<Option<(String, String, String)>> {
        match self.db_conn.query_row(
            "SELECT id, nonce, key FROM keystore WHERE tag = ?1",
            params![tag],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        ) {
            Ok((id, nonce, key)) => Ok(Some((id, nonce, key))),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => bail!("Query failed: {}", e),
        }
    }
}