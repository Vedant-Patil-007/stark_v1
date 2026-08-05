use rusqlite::Connection;
use crate::backup::{self, BackupReason};
use crate::error::{Result, StorageError};

/// A single forward migration. Never edit a released migration — add a new one.
struct Migration {
    version: i32,
    name: &'static str,
    sql: &'static str,
}

/// Ordered list of all migrations. Append only.
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "initial",
        sql: include_str!("../migrations/001_initial.sql"),
    },
    Migration {
        version: 2,
        name: "milestone_task_links",
        sql: include_str!("../migrations/002_milestone_task_links.sql"),
    },
];

/// Highest schema version this build understands.
pub fn target_version() -> i32 {
    MIGRATIONS.last().map(|m| m.version).unwrap_or(0)
}

pub fn current_version(conn: &Connection) -> Result<i32> {
    Ok(conn.query_row("PRAGMA user_version", [], |row| row.get(0))?)
}

#[derive(Debug)]
pub struct MigrationReport {
    pub from: i32,
    pub to: i32,
    pub applied: Vec<(i32, String)>,
    pub backup: Option<String>,
}

/// Bring the database up to `target_version()`.
/// Pure schema logic: no filesystem side effects. See `run_with_backup`.
pub fn run(conn: &mut Connection) -> Result<MigrationReport> {
    let from = current_version(conn)?;
    let to = target_version();

    if from > to {
        return Err(StorageError::SchemaTooNew {
            found: from,
            supported: to,
        });
    }

    let pending: Vec<&Migration> =
        MIGRATIONS.iter().filter(|m| m.version > from).collect();

    if pending.is_empty() {
        return Ok(MigrationReport {
            from,
            to,
            applied: Vec::new(),
            backup: None,
        });
    }

    let mut applied = Vec::new();
    for migration in pending {
        let tx = conn.transaction()?;
        tx.execute_batch(migration.sql)?;
        tx.pragma_update(None, "user_version", migration.version)?;
        tx.commit()?;
        applied.push((migration.version, migration.name.to_string()));
    }

    Ok(MigrationReport {
        from,
        to,
        applied,
        backup: None,
    })
}

/// Migrate, taking a mandatory pre-migration backup if any work is pending.
/// Use this from the application. Tests should use `run` directly.
pub fn run_with_backup(conn: &mut Connection) -> Result<MigrationReport> {
    let from = current_version(conn)?;
    let to = target_version();

    if from >= to {
        return run(conn);
    }

    let backup_path = backup::create(conn, BackupReason::PreMigration)?;
    let mut report = run(conn)?;
    report.backup = Some(backup_path.display().to_string());
    Ok(report)
}
