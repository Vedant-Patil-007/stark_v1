use rusqlite::Connection;
use crate::error::Result;
use crate::paths;

pub fn open() -> Result<Connection> {
    paths::ensure_dirs()?;
    let conn = Connection::open(paths::db_path()?)?;
    configure(&conn)?;
    Ok(conn)
}

/// In-memory connection for tests.
pub fn open_in_memory() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    configure(&conn)?;
    Ok(conn)
}

fn configure(conn: &Connection) -> Result<()> {
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.busy_timeout(std::time::Duration::from_secs(5))?;
    Ok(())
}