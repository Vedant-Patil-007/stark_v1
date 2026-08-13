use rusqlite::{Connection, params};
use stark_domain::{GoalId, MilestoneId, NewTask, Priority, Status, Task, TaskFilter, TaskId};
use crate::error::Result;
use crate::time_util::now_utc;

pub fn create(conn: &Connection, input: NewTask) -> Result<Task> {
    let now = now_utc();

    let task = Task {
        id: TaskId::new(),
        goal_id: input.goal_id,
        milestone_id: input.milestone_id,
        title: input.title,
        description: input.description,
        due_date: input.due_date,
        scheduled_date: input.scheduled_date,
        estimated_minutes: input.estimated_minutes,
        priority: input.priority,
        status: Status::NotStarted,
        created_at: now.clone(),
        updated_at: now,
        completed_at: None,
        deleted_at: None,
    };

    conn.execute(
        "INSERT INTO task (id, goal_id, milestone_id, title, description,
                           due_date, scheduled_date, estimated_minutes,
                           priority, status, created_at, updated_at,
                           completed_at, deleted_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, NULL, NULL)",
        params![
            task.id.as_str(),
            task.goal_id.as_ref().map(|g| g.0.clone()),
            task.milestone_id.as_ref().map(|m| m.0.clone()),
            task.title,
            task.description,
            task.due_date,
            task.scheduled_date,
            task.estimated_minutes,
            task.priority.as_db(),
            task.status.as_db(),
            task.created_at,
            task.updated_at,
        ],
    )?;

    Ok(task)
}

pub fn list(conn: &Connection, filter: &TaskFilter) -> Result<Vec<Task>> {
    let mut sql = String::from(
        "SELECT id, goal_id, milestone_id, title, description,
                due_date, scheduled_date, estimated_minutes,
                priority, status, created_at, updated_at,
                completed_at, deleted_at
         FROM task
         WHERE deleted_at IS NULL",
    );

    let mut args: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(g) = &filter.goal_id {
        sql.push_str(" AND goal_id = ?");
        args.push(Box::new(g.0.clone()));
    }
    if let Some(m) = &filter.milestone_id {
        sql.push_str(" AND milestone_id = ?");
        args.push(Box::new(m.0.clone()));
    }
    if let Some(d) = &filter.scheduled_date {
        sql.push_str(" AND scheduled_date = ?");
        args.push(Box::new(d.clone()));
    }
    if !filter.include_completed {
        sql.push_str(" AND status NOT IN ('COMPLETED','CANCELLED')");
    }

    sql.push_str(" ORDER BY COALESCE(scheduled_date, due_date, '9999-12-31'), created_at");

    let mut stmt = conn.prepare(&sql)?;
    let refs: Vec<&dyn rusqlite::ToSql> = args.iter().map(|a| a.as_ref()).collect();
    let rows = stmt.query_map(refs.as_slice(), row_to_task)?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn set_status(conn: &Connection, id: &TaskId, status: Status) -> Result<bool> {
    let now = now_utc();
    let completed_at = if status == Status::Completed {
        Some(now.clone())
    } else {
        None
    };

    let n = conn.execute(
        "UPDATE task SET status = ?1, completed_at = ?2, updated_at = ?3
         WHERE id = ?4 AND deleted_at IS NULL",
        params![status.as_db(), completed_at, now, id.as_str()],
    )?;
    Ok(n > 0)
}

pub fn set_scheduled_date(
    conn: &Connection,
    id: &TaskId,
    date: Option<String>,
) -> Result<bool> {
    let n = conn.execute(
        "UPDATE task SET scheduled_date = ?1, updated_at = ?2
         WHERE id = ?3 AND deleted_at IS NULL",
        params![date, now_utc(), id.as_str()],
    )?;
    Ok(n > 0)
}

pub fn soft_delete(conn: &Connection, id: &TaskId) -> Result<bool> {
    let n = conn.execute(
        "UPDATE task SET deleted_at = ?1, updated_at = ?1
         WHERE id = ?2 AND deleted_at IS NULL",
        params![now_utc(), id.as_str()],
    )?;
    Ok(n > 0)
}


fn row_to_task(row: &rusqlite::Row<'_>) -> rusqlite::Result<Task> {
    let priority_str: String = row.get(8)?;
    let status_str: String = row.get(9)?;

    Ok(Task {
        id: TaskId::from(row.get::<_, String>(0)?),
        goal_id: row.get::<_, Option<String>>(1)?.map(GoalId::from),
        milestone_id: row.get::<_, Option<String>>(2)?.map(MilestoneId::from),
        title: row.get(3)?,
        description: row.get(4)?,
        due_date: row.get(5)?,
        scheduled_date: row.get(6)?,
        estimated_minutes: row.get(7)?,
        priority: Priority::from_db(&priority_str).unwrap_or(Priority::Medium),
        status: Status::from_db(&status_str).unwrap_or(Status::NotStarted),
        created_at: row.get(10)?,
        updated_at: row.get(11)?,
        completed_at: row.get(12)?,
        deleted_at: row.get(13)?,
    })
}
/// Tasks whose scheduled_date OR due_date falls within [from, to].
pub fn list_in_range(conn: &Connection, from: &str, to: &str) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, goal_id, milestone_id, title, description,
                due_date, scheduled_date, estimated_minutes,
                priority, status, created_at, updated_at,
                completed_at, deleted_at
         FROM task
         WHERE deleted_at IS NULL
           AND (
             (scheduled_date IS NOT NULL AND scheduled_date >= ?1 AND scheduled_date <= ?2)
             OR (due_date IS NOT NULL AND due_date >= ?1 AND due_date <= ?2)
           )
         ORDER BY COALESCE(scheduled_date, due_date), created_at",
    )?;
    let rows = stmt.query_map(params![from, to], row_to_task)?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}