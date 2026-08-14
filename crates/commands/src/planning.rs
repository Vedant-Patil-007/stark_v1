use rusqlite::Connection;
use stark_planning::{analyze, Analysis};
use stark_storage::snapshot_builder;
use crate::error::Result;

pub fn analyze_plan(conn: &Connection, today: &str) -> Result<Analysis> {
    let snapshot = snapshot_builder::build(conn, today)?;
    Ok(analyze(&snapshot))
}