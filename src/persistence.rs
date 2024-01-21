use rusqlite::Connection;
use crate::common::utils::get_user_path;

pub mod migrations;
mod key_store;
mod file_system;

pub struct SqlLiteConnection {
    pub db_conn: Connection,
}

impl SqlLiteConnection {
    pub fn new() -> anyhow::Result<Self> {
        let mut path = get_user_path()?;
        path.push("db.db3");
        let conn = Connection::open(path).unwrap();
        Ok(Self {
            db_conn: conn
        })
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        let migrations = migrations::get_migrations();
        self.db_conn.pragma_update(None, "journal_mode", "WAL").unwrap();
        migrations.to_latest(&mut self.db_conn)?;
        Ok(())
    }
}
