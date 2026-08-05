use rusqlite::{Connection, params};
use stark_domain::{GoalId, Milestone, MilestoneId, NewMilestone, Status};
use crate::error::Result;
use crate::time_util::now_utc;

pub fn create(conn: &Connection, input: NewMilestone) -> Result<Milestone> {
    let id = MilestoneId::new();
    let now = now_utc();

    // Append to the end of this goal's milestone list.
    let next_index: i64 = conn.query_row(
        "SELECT COALESCE(MAX(order_index) + 1, 0) FROM milestone
         WHERE goal_id = ?1 AND deleted_at IS NULL",
        params![input.goal_id.as_str()],
        |r| r.get(0),
    )?;

    let milestone = Milestone {
        id,
        goal_id: input.goal_id,
        title: input.title,
        description: input.description,
        target_date: input.target_date,
        status: Status::NotStarted,
        order_index: next_index,
        created_at: now.clone(),
        updated_at: now,
        deleted_at: None,
    };

    conn.execute(
        "INSERT INTO milestone (id, goal_id, title, description, target_date,
                                status, order_index, created_at, updated_at, deleted_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL)",
        params![
            milestone.id.as_str(),
            milestone.goal_id.as_str(),
            milestone.title,
            milestone.description,
            milestone.target_date,
            milestone.status.as_db(),
            milestone.order_index,
            milestone.created_at,
            milestone.updated_at,
        ],
    )?;

    Ok(milestone)
}

pub fn list_for_goal(conn: &Connection, goal_id: &GoalId) -> Result<Vec<Milestone>> {
    let mut stmt = conn.prepare(
        "SELECT id, goal_id, title, description, target_date,
                status, order_index, created_at, updated_at, deleted_at
         FROM milestone
         WHERE goal_id = ?1 AND deleted_at IS NULL
         ORDER BY order_index",
    )?;

    let rows = stmt.query_map(params![goal_id.as_str()], row_to_milestone)?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn get(conn: &Connection, id: &MilestoneId) -> Result<Option<Milestone>> {
    let mut stmt = conn.prepare(
        "SELECT id, goal_id, title, description, target_date,
                status, order_index, created_at, updated_at, deleted_at
         FROM milestone
         WHERE id = ?1 AND deleted_at IS NULL",
    )?;

    let mut rows = stmt.query_map(params![id.as_str()], row_to_milestone)?;
    match rows.next() {
        Some(r) => Ok(Some(r?)),
        None => Ok(None),
    }
}

pub fn set_status(conn: &Connection, id: &MilestoneId, status: Status) -> Result<bool> {
    let n = conn.execute(
        "UPDATE milestone SET status = ?1, updated_at = ?2
         WHERE id = ?3 AND deleted_at IS NULL",
        params![status.as_db(), now_utc(), id.as_str()],
    )?;
    Ok(n > 0)
}

pub fn soft_delete(conn: &Connection, id: &MilestoneId) -> Result<bool> {
    let n = conn.execute(
        "UPDATE milestone SET deleted_at = ?1, updated_at = ?1
         WHERE id = ?2 AND deleted_at IS NULL",
        params![now_utc(), id.as_str()],
    )?;
    Ok(n > 0)
}

fn row_to_milestone(row: &rusqlite::Row<'_>) -> rusqlite::Result<Milestone> {
    let status_str: String = row.get(5)?;
    Ok(Milestone {
        id: MilestoneId::from(row.get::<_, String>(0)?),
        goal_id: GoalId::from(row.get::<_, String>(1)?),
        title: row.get(2)?,
        description: row.get(3)?,
        target_date: row.get(4)?,
        status: Status::from_db(&status_str).unwrap_or(Status::NotStarted),
        order_index: row.get(6)?,
        created_at: row.get(7)?,
        updated_at: row.get(8)?,
        deleted_at: row.get(9)?,
    })
}