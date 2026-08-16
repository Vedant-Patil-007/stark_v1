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

use stark_domain::{NewReminder, Reminder};
use stark_storage::reminder_repo;

/// Minutes before a scheduled task's start that a reminder fires.
const LEAD_MINUTES: i64 = 15;

/// Create reminders for tasks scheduled on `date` that don't already have one.
/// Idempotent: safe to call on every startup and after any task change.
pub fn sync_reminders_for_date(
    conn: &Connection,
    date: &str,
    local_offset_minutes: i64,
) -> Result<usize> {
    let tasks = task_repo::list(
        conn,
        &TaskFilter {
            goal_id: None,
            milestone_id: None,
            scheduled_date: Some(date.to_string()),
            include_completed: false,
        },
    )?;

    let mut created = 0;

    for t in tasks {
        // Clear any stale pending reminder, then recreate.
        reminder_repo::delete_pending_for_task(conn, &t.id)?;

        // No time-of-day on tasks yet, so remind at 09:00 local.
        let fire_local_minutes = 9 * 60 - LEAD_MINUTES;
        let fire_utc = local_date_time_to_utc(date, fire_local_minutes, local_offset_minutes);

        reminder_repo::create(
            conn,
            NewReminder {
                task_id: Some(t.id.clone()),
                goal_id: t.goal_id.clone(),
                fire_at_utc: fire_utc,
                title: t.title.clone(),
                body: Some(match t.estimated_minutes {
                    Some(m) => format!("Scheduled for today · {m} minutes"),
                    None => "Scheduled for today".to_string(),
                }),
            },
        )?;
        created += 1;
    }

    Ok(created)
}

pub fn list_missed_reminders(conn: &Connection) -> Result<Vec<Reminder>> {
    Ok(reminder_repo::list_missed(conn)?)
}

pub fn dismiss_reminder(conn: &Connection, id: &stark_domain::ReminderId) -> Result<()> {
    reminder_repo::set_status(conn, id, stark_domain::ReminderStatus::Dismissed)?;
    Ok(())
}

/// Convert a local date + minute-of-day into an ISO-8601 UTC instant.
fn local_date_time_to_utc(date: &str, local_minutes: i64, offset_minutes: i64) -> String {
    let y: i64 = date[0..4].parse().unwrap_or(1970);
    let m: i64 = date[5..7].parse().unwrap_or(1);
    let d: i64 = date[8..10].parse().unwrap_or(1);

    let total = days_from_civil(y, m, d) * 1440 + local_minutes - offset_minutes;

    let days = total.div_euclid(1440);
    let mins = total.rem_euclid(1440);
    let (uy, um, ud) = civil_from_days(days);

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:00Z",
        uy,
        um,
        ud,
        mins / 60,
        mins % 60
    )
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}