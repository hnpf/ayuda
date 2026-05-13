use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn seek(conn: &Connection, dest: &str) -> Result<Option<PathBuf>> {
    //check aliases
    let mut stmt = conn.prepare("SELECT path FROM aliases WHERE name = ?")?;
    let mut rows = stmt.query([dest])?;
    if let Some(row) = rows.next()? {
        let path: String = row.get(0)?;
        return Ok(Some(PathBuf::from(path)));
    }

    // check hist (simplified fuzzy....)
    let mut stmt = conn.prepare("SELECT path FROM history WHERE path LIKE ? ORDER BY last_access DESC LIMIT 1")?;
    let mut rows = stmt.query([format!("%{}%", dest)])?;
    if let Some(row) = rows.next()? {
        let path: String = row.get(0)?;
        return Ok(Some(PathBuf::from(path)));
    }

    Ok(None)
}
