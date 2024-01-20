use rusqlite_migration::{M, Migrations};

pub fn get_migrations<'m>() -> Migrations<'m> {
    let vec = vec![
        M::up(
            "CREATE TABLE keystore (
                    id	TEXT NOT NULL,
                    nonce	TEXT NOT NULL,
                    key	TEXT NOT NULL,
                    PRIMARY KEY(id)
                )"
        ),
        M::up(
            "CREATE UNIQUE INDEX keystore_id ON keystore (
                    id	ASC
                )"
        ),
    ];
    Migrations::new(vec)
}