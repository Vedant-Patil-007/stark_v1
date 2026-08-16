use rusqlite::{Connection, params};
use stark_domain::{GoalId, NewReminder, Reminder, ReminderId, ReminderStatus, TaskId};
use crate::error::Result;
use crate::time_util::now_utc;

pub fn create(conn: &Connection, input: NewReminder) -> Result<Reminder> {
    let reminder = Reminder {
        id: ReminderId::new(),
        task_id: input.task_id,
        goal_id: input.goal_id,
        fire_at_utc: input.fire_at_utc,
        title: input.title,
        body: input.body,
        status: ReminderStatus::Pending,
        fired_at: None,
        created_at: now_utc(),
    };

    conn.execute(
        "INSERT INTO reminder (id, task_id, goal_id, fire_at_utc, title, body,
                               status, fired_at, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, ?8)",
        params![
            reminder.id.as_str(),
            reminder.task_id.as_ref().map(|t| t.0.clone()),
            reminder.goal_id.as_ref().map(|g| g.0.clone()),
            reminder.fire_at_utc,
            reminder.title,
            reminder.body,
            reminder.status.as_db(),
            reminder.created_at,
        ],
    )?;

    Ok(reminder)
}

/// The next pending reminder due at or after `now`. Drives the scheduler timer.
pub fn next_pending(conn: &Connection, now_utc_str: &str) -> Result<Option<Reminder>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_id, goal_id, fire_at_utc, title, body, status, fired_at, created_at
         FROM reminder
         WHERE status = 'PENDING' AND fire_at_utc >= ?1
         ORDER BY fire_at_utc
         LIMIT 1",
    )?;
    let mut rows = stmt.query_map(params![now_utc_str], row_to_reminder)?;
    match rows.next() {
        Some(r) => Ok(Some(r?)),
        None => Ok(None),
    }
}

/// Pending reminders whose time has already passed.
pub fn overdue_pending(conn: &Connection, now_utc_str: &str) -> Result<Vec<Reminder>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_id, goal_id, fire_at_utc, title, body, status, fired_at, created_at
         FROM reminder
         WHERE status = 'PENDING' AND fire_at_utc < ?1
         ORDER BY fire_at_utc",
    )?;
    let rows = stmt.query_map(params![now_utc_str], row_to_reminder)?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn set_status(
    conn: &Connection,
    id: &ReminderId,
    status: ReminderStatus,
) -> Result<bool> {
    let fired_at = match status {
        ReminderStatus::Fired => Some(now_utc()),
        _ => None,
    };
    let n = conn.execute(
        "UPDATE reminder SET status = ?1, fired_at = COALESCE(?2, fired_at) WHERE id = ?3",
        params![status.as_db(), fired_at, id.as_str()],
    )?;
    Ok(n > 0)
}

/// Reminders marked MISSED and not yet dismissed — shown as a startup digest.
pub fn list_missed(conn: &Connection) -> Result<Vec<Reminder>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_id, goal_id, fire_at_utc, title, body, status, fired_at, created_at
         FROM reminder
         WHERE status = 'MISSED'
         ORDER BY fire_at_utc DESC
         LIMIT 20",
    )?;
    let rows = stmt.query_map([], row_to_reminder)?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn delete(conn: &Connection, id: &ReminderId) -> Result<bool> {
    let n = conn.execute("DELETE FROM reminder WHERE id = ?1", params![id.as_str()])?;
    Ok(n > 0)
}

/// Remove pending reminders for a task. Used when a task is rescheduled or completed.
pub fn delete_pending_for_task(conn: &Connection, task_id: &TaskId) -> Result<usize> {
    Ok(conn.execute(
        "DELETE FROM reminder WHERE task_id = ?1 AND status = 'PENDING'",
        params![task_id.as_str()],
    )?)
}

fn row_to_reminder(row: &rusqlite::Row<'_>) -> rusqlite::Result<Reminder> {
    let status_str: String = row.get(6)?;
    Ok(Reminder {
        id: ReminderId::from(row.get::<_, String>(0)?),
        task_id: row.get::<_, Option<String>>(1)?.map(TaskId::from),
        goal_id: row.get::<_, Option<String>>(2)?.map(GoalId::from),
        fire_at_utc: row.get(3)?,
        title: row.get(4)?,
        body: row.get(5)?,
        status: ReminderStatus::from_db(&status_str).unwrap_or(ReminderStatus::Pending),
        fired_at: row.get(7)?,
        created_at: row.get(8)?,
    })
}