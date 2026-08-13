use rusqlite::Connection;
use stark_domain::{GoalId, LogEntry, LogEntryId, NewLogEntry, TaskId};
use stark_storage::{log_repo};
use crate::error::{CommandError, Result};
use crate::validate;

pub fn create_log_entry(conn: &Connection, mut input: NewLogEntry) -> Result<LogEntry> {
    let date = validate::optional_date(Some(input.log_date.clone()), "log_date")?
        .ok_or_else(|| CommandError::Validation("log_date is required".into()))?;
    input.log_date = date;

    input.activity = validate::title(&input.activity)?;
    input.notes = validate::optional_description(input.notes)?;

    if let Some(mins) = input.duration_minutes {
        if mins <= 0 {
            return Err(CommandError::Validation(
                "duration must be positive".into(),
            ));
        }
        if mins > 24 * 60 {
            return Err(CommandError::Validation(
                "a single log entry cannot exceed 24 hours".into(),
            ));
        }
    }

    Ok(log_repo::create(conn, input)?)
}

pub fn list_log_for_date(conn: &Connection, date: &str) -> Result<Vec<LogEntry>> {
    Ok(log_repo::list_for_date(conn, date)?)
}

pub fn list_log_for_range(conn: &Connection, from: &str, to: &str) -> Result<Vec<LogEntry>> {
    if from > to {
        return Err(CommandError::Validation(
            "range start cannot be after range end".into(),
        ));
    }
    Ok(log_repo::list_for_range(conn, from, to)?)
}

pub fn task_actual_minutes(conn: &Connection, task_id: &TaskId) -> Result<i64> {
    Ok(log_repo::minutes_for_task(conn, task_id)?)
}

pub fn goal_actual_minutes(
    conn: &Connection,
    goal_id: &GoalId,
    from: &str,
    to: &str,
) -> Result<i64> {
    Ok(log_repo::minutes_for_goal(conn, goal_id, from, to)?)
}

pub fn delete_log_entry(conn: &Connection, id: &LogEntryId) -> Result<()> {
    if log_repo::delete(conn, id)? {
        Ok(())
    } else {
        Err(CommandError::NotFound(format!("log entry {id}")))
    }
}