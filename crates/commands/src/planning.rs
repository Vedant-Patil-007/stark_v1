use rusqlite::Connection;
use stark_planning::{analyze, Analysis};
use stark_storage::snapshot_builder;
use crate::error::Result;
use stark_domain::{Task, TaskFilter};
use stark_storage::task_repo;

pub fn analyze_plan(conn: &Connection, today: &str) -> Result<Analysis> {
    let snapshot = snapshot_builder::build(conn, today)?;
    Ok(analyze(&snapshot))
}
/// Tasks scheduled for a given date, plus anything overdue.
pub fn today_tasks(conn: &Connection, today: &str) -> Result<Vec<Task>> {
    let scheduled = task_repo::list(
        conn,
        &TaskFilter {
            goal_id: None,
            milestone_id: None,
            scheduled_date: Some(today.to_string()),
            include_completed: true,
        },
    )?;
    Ok(scheduled)
}

/// Outstanding tasks whose due date has already passed.
pub fn overdue_tasks(conn: &Connection, today: &str) -> Result<Vec<Task>> {
    let all = task_repo::list(
        conn,
        &TaskFilter {
            goal_id: None,
            milestone_id: None,
            scheduled_date: None,
            include_completed: false,
        },
    )?;
    Ok(all
        .into_iter()
        .filter(|t| match &t.due_date {
            Some(d) => d.as_str() < today,
            None => false,
        })
        .collect())
}