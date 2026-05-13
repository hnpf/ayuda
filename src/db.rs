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
            path TEXT PRIMARY KEY,
            last_access DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS config (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    // default first phase
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES ('sass', '1')",
        [],
    )?;

    Ok(conn)
}

pub fn update_config(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO config (key, value) VALUES (?, ?)",
        [key, value],
    )?;
    Ok(())
}

pub fn get_config(conn: &Connection, key: &str) -> Result<String> {
    let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?")?;
    let mut rows = stmt.query([key])?;
    if let Some(row) = rows.next()? {
        Ok(row.get(0)?)
    } else {
        Ok("".to_string())
    }
}
