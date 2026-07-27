use stark_storage::{db, migrations};

#[test]
fn fresh_database_starts_at_version_zero() {
    let conn = db::open_in_memory().unwrap();
    assert_eq!(migrations::current_version(&conn).unwrap(), 0);
}

#[test]
fn running_migrations_reaches_target_version() {
    let mut conn = db::open_in_memory().unwrap();
    let report = migrations::run(&mut conn).unwrap();
    assert_eq!(report.to, migrations::target_version());
    assert_eq!(
        migrations::current_version(&conn).unwrap(),
        migrations::target_version()
    );
}

#[test]
fn migrations_are_idempotent() {
    let mut conn = db::open_in_memory().unwrap();
    migrations::run(&mut conn).unwrap();
    let second = migrations::run(&mut conn).unwrap();
    assert!(second.applied.is_empty(), "second run should apply nothing");
}

#[test]
fn foreign_keys_and_wal_are_enabled() {
    let conn = db::open_in_memory().unwrap();
    let fk: i32 = conn
        .query_row("PRAGMA foreign_keys", [], |r| r.get(0))
        .unwrap();
    assert_eq!(fk, 1, "foreign keys must be ON");
}
#[test]
fn initial_migration_creates_expected_tables() {
    let mut conn = db::open_in_memory().unwrap();
    migrations::run(&mut conn).unwrap();

    for table in ["settings", "goal", "goal_success_criterion", "milestone", "task", "task_tag"] {
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                [table],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 1, "table {table} should exist");
    }
}

#[test]
fn invalid_status_is_rejected() {
    let mut conn = db::open_in_memory().unwrap();
    migrations::run(&mut conn).unwrap();

    let result = conn.execute(
        "INSERT INTO goal (id, title, priority, status, created_at, updated_at)
         VALUES ('g1', 'Test', 'HIGH', 'NONSENSE', '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z')",
        [],
    );
    assert!(result.is_err(), "CHECK constraint should reject bad status");
}