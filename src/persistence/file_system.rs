use anyhow::bail;
use anyhow::Result;
use rusqlite::{Error, params};
use crate::persistence::SqlLiteConnection;

impl crate::file_system::FileSystem for SqlLiteConnection {
    fn save_path(&self, path: &str, key: &str) -> Result<()> {
        self.db_conn.execute(
            "INSERT INTO filesystem (id, path) VALUES (?1, ?2)",
            params![key, path],
        )?;
        Ok(())
    }

    fn get_path(&self, path: &str) -> Result<Option<String>> {
        match self.db_conn.query_row(
            "SELECT id FROM filesystem WHERE path = ?1",
            params![path],
            |row| row.get(0),
        ) {
            Ok(key) => Ok(Some(key)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => bail!("Query failed: {}", e),
        }
    }
}