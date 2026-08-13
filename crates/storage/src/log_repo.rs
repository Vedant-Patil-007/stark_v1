use rusqlite::{Connection, params};
use stark_domain::{GoalId, LogEntry, LogEntryId, MilestoneId, NewLogEntry, TaskId};
use crate::error::Result;
use crate::time_util::now_utc;

pub fn create(conn: &Connection, input: NewLogEntry) -> Result<LogEntry> {
    let now = now_utc();

    let entry = LogEntry {
        id: LogEntryId::new(),
        log_date: input.log_date,
        task_id: input.task_id,
        milestone_id: input.milestone_id,
        goal_id: input.goal_id,
        activity: input.activity,
        duration_minutes: input.duration_minutes,
        category: input.category,
        notes: input.notes,
        created_at: now.clone(),
        updated_at: now,
    };

    conn.execute(
        "INSERT INTO daily_log_entry (id, log_date, task_id, milestone_id, goal_id,
                                      activity, duration_minutes, category, notes,
                                      created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            entry.id.as_str(),
            entry.log_date,
            entry.task_id.as_ref().map(|t| t.0.clone()),
            entry.milestone_id.as_ref().map(|m| m.0.clone()),
            entry.goal_id.as_ref().map(|g| g.0.clone()),
            entry.activity,
            entry.duration_minutes,
            entry.category,
            entry.notes,
            entry.created_at,
            entry.updated_at,
        ],
    )?;

    Ok(entry)
}

pub fn list_for_date(conn: &Connection, date: &str) -> Result<Vec<LogEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, log_date, task_id, milestone_id, goal_id,
                activity, duration_minutes, category, notes,
                created_at, updated_at
         FROM daily_log_entry
         WHERE log_date = ?1
         ORDER BY created_at",
    )?;
    let rows = stmt.query_map(params![date], row_to_entry)?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn list_for_range(conn: &Connection, from: &str, to: &str) -> Result<Vec<LogEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, log_date, task_id, milestone_id, goal_id,
                activity, duration_minutes, category, notes,
                created_at, updated_at
         FROM daily_log_entry
         WHERE log_date >= ?1 AND log_date <= ?2
         ORDER BY log_date, created_at",
    )?;
    let rows = stmt.query_map(params![from, to], row_to_entry)?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

/// Total logged minutes for a task. This is the derived "actual duration".
pub fn minutes_for_task(conn: &Connection, task_id: &TaskId) -> Result<i64> {
    Ok(conn.query_row(
        "SELECT COALESCE(SUM(duration_minutes), 0) FROM daily_log_entry WHERE task_id = ?1",
        params![task_id.as_str()],
        |r| r.get(0),
    )?)
}

/// Total logged minutes for a goal, across a date range.
pub fn minutes_for_goal(
    conn: &Connection,
    goal_id: &GoalId,
    from: &str,
    to: &str,
) -> Result<i64> {
    Ok(conn.query_row(
        "SELECT COALESCE(SUM(duration_minutes), 0) FROM daily_log_entry
         WHERE goal_id = ?1 AND log_date >= ?2 AND log_date <= ?3",
        params![goal_id.as_str(), from, to],
        |r| r.get(0),
    )?)
}

pub fn delete(conn: &Connection, id: &LogEntryId) -> Result<bool> {
    let n = conn.execute(
        "DELETE FROM daily_log_entry WHERE id = ?1",
        params![id.as_str()],
    )?;
    Ok(n > 0)
}

fn row_to_entry(row: &rusqlite::Row<'_>) -> rusqlite::Result<LogEntry> {
    Ok(LogEntry {
        id: LogEntryId::from(row.get::<_, String>(0)?),
        log_date: row.get(1)?,
        task_id: row.get::<_, Option<String>>(2)?.map(TaskId::from),
        milestone_id: row.get::<_, Option<String>>(3)?.map(MilestoneId::from),
        goal_id: row.get::<_, Option<String>>(4)?.map(GoalId::from),
        activity: row.get(5)?,
        duration_minutes: row.get(6)?,
        category: row.get(7)?,
        notes: row.get(8)?,
        created_at: row.get(9)?,
        updated_at: row.get(10)?,
    })
}