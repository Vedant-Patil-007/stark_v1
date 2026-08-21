//! API key storage.
//!
//! Originally used the OS credential store via `keyring`, but on Windows the
//! write reported success while the read returned no entry. The key now lives
//! in the app's own settings table — same machine-local trust boundary, one
//! fewer dependency, and it is never written to the repo or to localStorage.

use rusqlite::{Connection, params};
use crate::error::{AiError, Result};

fn key_name(provider: &str) -> String {
    format!("ai_key:{provider}")
}

pub fn store_key(conn: &Connection, provider: &str, key: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO settings (key, value_json) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value_json = excluded.value_json",
        params![key_name(provider), key],
    )
    .map_err(|e| AiError::Provider(format!("storing key: {e}")))?;
    Ok(())
}

pub fn load_key(conn: &Connection, provider: &str) -> Result<Option<String>> {
    let mut stmt = conn
        .prepare("SELECT value_json FROM settings WHERE key = ?1")
        .map_err(|e| AiError::Provider(format!("loading key: {e}")))?;

    let mut rows = stmt
        .query_map(params![key_name(provider)], |r| r.get::<_, String>(0))
        .map_err(|e| AiError::Provider(format!("loading key: {e}")))?;

    match rows.next() {
        Some(Ok(k)) if !k.trim().is_empty() => Ok(Some(k)),
        Some(Err(e)) => Err(AiError::Provider(format!("loading key: {e}"))),
        _ => Ok(None),
    }
}

pub fn delete_key(conn: &Connection, provider: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM settings WHERE key = ?1",
        params![key_name(provider)],
    )
    .map_err(|e| AiError::Provider(format!("deleting key: {e}")))?;
    Ok(())
}