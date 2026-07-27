use rusqlite::Connection;
use std::path::PathBuf;
use time::OffsetDateTime;
use crate::error::{Result, StorageError};
use crate::paths;

/// Reason a backup was taken. Becomes part of the filename.
#[derive(Debug, Clone, Copy)]
pub enum BackupReason {
    Daily,
    PreMigration,
    Manual,
}

impl BackupReason {
    fn tag(self) -> &'static str {
        match self {
            BackupReason::Daily => "daily",
            BackupReason::PreMigration => "premigration",
            BackupReason::Manual => "manual",
        }
    }
}

/// Snapshot the database using VACUUM INTO. Safe on a live connection.
pub fn create(conn: &Connection, reason: BackupReason) -> Result<PathBuf> {
    paths::ensure_dirs()?;

    let now = OffsetDateTime::now_utc();
    let name = format!(
        "stark-{:04}{:02}{:02}-{:02}{:02}{:02}-{}.db",
        now.year(),
        now.month() as u8,
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        reason.tag()
    );

    let dest = paths::backup_dir()?.join(&name);

    // VACUUM INTO refuses to overwrite, so a collision is a real error.
    conn.execute("VACUUM INTO ?1", [dest.to_string_lossy().as_ref()])?;

    Ok(dest)
}

/// Take a daily backup only if none exists for today.
pub fn create_daily_if_needed(conn: &Connection) -> Result<Option<PathBuf>> {
    let now = OffsetDateTime::now_utc();
    let today = format!(
        "stark-{:04}{:02}{:02}",
        now.year(),
        now.month() as u8,
        now.day()
    );

    let dir = paths::backup_dir()?;
    paths::ensure_dirs()?;

    let entries = std::fs::read_dir(&dir).map_err(|source| StorageError::Io {
        path: dir.clone(),
        source,
    })?;

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with(&today) || name.starts_with("stark-") && name.contains(&today[6..]) {
            if name.starts_with(&today) && name.ends_with("-daily.db") {
                return Ok(None);
            }
        }
    }

    Ok(Some(create(conn, BackupReason::Daily)?))
}

/// Keep the most recent `keep` daily backups; delete older ones.
/// Pre-migration and manual backups are never pruned.
pub fn prune_daily(keep: usize) -> Result<usize> {
    let dir = paths::backup_dir()?;
    let entries = std::fs::read_dir(&dir).map_err(|source| StorageError::Io {
        path: dir.clone(),
        source,
    })?;

    let mut daily: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with("-daily.db"))
                .unwrap_or(false)
        })
        .collect();

    // Filenames are timestamped, so lexical sort == chronological sort.
    daily.sort();

    let mut removed = 0;
    if daily.len() > keep {
        for path in &daily[..daily.len() - keep] {
            if std::fs::remove_file(path).is_ok() {
                removed += 1;
            }
        }
    }

    Ok(removed)
}