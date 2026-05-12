use rusqlite::{Connection, Result};

pub fn record(conn: &Connection, path: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO history (path, last_access) 
         VALUES (?, CURRENT_TIMESTAMP)
         ON CONFLICT(id) DO UPDATE SET last_access = CURRENT_TIMESTAMP",
        [path],
    )?;
    Ok(())
}
