use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn init_db() -> Result<Connection> {
    let mut db_path = home::home_dir().unwrap_or_else(|| PathBuf::from("."));
    db_path.push(".ayuda.db");
    
    let conn = Connection::open(db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS aliases (
            name TEXT PRIMARY KEY,
            path TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            last_access DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    Ok(conn)
}
