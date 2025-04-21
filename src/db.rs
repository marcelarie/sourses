// src/db.rs

use anyhow::Result;
use dirs::home_dir;
use rusqlite::{params, Connection};

/// Opens (or creates) the ~/.sourses/sourses.db SQLite database.
pub fn get_connection() -> Result<Connection> {
    let mut path = home_dir()
        .expect("Failed to get home directory")
        .join(".sourses");

    std::fs::create_dir_all(&path)?;
    path.push("sourses.db");

    let conn = Connection::open(path)?;
    Ok(conn)
}

pub fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS items (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            type      TEXT NOT NULL,
            text      TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}

pub fn insert_item(conn: &Connection, item_type: &str, text: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO items (type, text) VALUES (?1, ?2)",
        params![item_type, text],
    )?;
    Ok(())
}
