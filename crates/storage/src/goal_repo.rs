use rusqlite::{Connection, params};
use stark_domain::{Goal, GoalId, NewGoal, Priority, Status, SuccessCriterion};
use stark_domain::ids::CriterionId;
use crate::error::Result;
use crate::time_util::now_utc;

pub fn create(conn: &mut Connection, input: NewGoal) -> Result<Goal> {
    let id = GoalId::new();
    let now = now_utc();

    let goal = Goal {
        id: id.clone(),
        title: input.title,
        description: input.description,
        start_date: input.start_date,
        target_date: input.target_date,
        priority: input.priority,
        status: Status::NotStarted,
        estimated_effort_minutes: input.estimated_effort_minutes,
        created_at: now.clone(),
        updated_at: now.clone(),
        deleted_at: None,
    };

    let tx = conn.transaction()?;

    tx.execute(
        "INSERT INTO goal (id, title, description, start_date, target_date,
                           priority, status, estimated_effort_minutes,
                           created_at, updated_at, deleted_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, NULL)",
        params![
            goal.id.as_str(),
            goal.title,
            goal.description,
            goal.start_date,
            goal.target_date,
            goal.priority.as_db(),
            goal.status.as_db(),
            goal.estimated_effort_minutes,
            goal.created_at,
            goal.updated_at,
        ],
    )?;

    for (idx, text) in input.success_criteria.iter().enumerate() {
        tx.execute(
            "INSERT INTO goal_success_criterion (id, goal_id, text, is_met, met_at, order_index)
             VALUES (?1, ?2, ?3, 0, NULL, ?4)",
            params![
                CriterionId::new().as_str(),
                goal.id.as_str(),
                text,
                idx as i64,
            ],
        )?;
    }

    tx.commit()?;
    Ok(goal)
}

pub fn list(conn: &Connection) -> Result<Vec<Goal>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, start_date, target_date,
                priority, status, estimated_effort_minutes,
                created_at, updated_at, deleted_at
         FROM goal
         WHERE deleted_at IS NULL
         ORDER BY created_at DESC",
    )?;

    let rows = stmt.query_map([], row_to_goal)?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn get(conn: &Connection, id: &GoalId) -> Result<Option<Goal>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, description, start_date, target_date,
                priority, status, estimated_effort_minutes,
                created_at, updated_at, deleted_at
         FROM goal
         WHERE id = ?1 AND deleted_at IS NULL",
    )?;

    let mut rows = stmt.query_map(params![id.as_str()], row_to_goal)?;
    match rows.next() {
        Some(r) => Ok(Some(r?)),
        None => Ok(None),
    }
}

pub fn soft_delete(conn: &Connection, id: &GoalId) -> Result<bool> {
    let n = conn.execute(
        "UPDATE goal SET deleted_at = ?1, updated_at = ?1
         WHERE id = ?2 AND deleted_at IS NULL",
        params![now_utc(), id.as_str()],
    )?;
    Ok(n > 0)
}

pub fn criteria_for(conn: &Connection, goal_id: &GoalId) -> Result<Vec<SuccessCriterion>> {
    let mut stmt = conn.prepare(
        "SELECT id, goal_id, text, is_met, met_at, order_index
         FROM goal_success_criterion
         WHERE goal_id = ?1
         ORDER BY order_index",
    )?;

    let rows = stmt.query_map(params![goal_id.as_str()], |row| {
        Ok(SuccessCriterion {
            id: CriterionId::from(row.get::<_, String>(0)?),
            goal_id: GoalId::from(row.get::<_, String>(1)?),
            text: row.get(2)?,
            is_met: row.get::<_, i64>(3)? != 0,
            met_at: row.get(4)?,
            order_index: row.get(5)?,
        })
    })?;

    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

fn row_to_goal(row: &rusqlite::Row<'_>) -> rusqlite::Result<Goal> {
    let priority_str: String = row.get(5)?;
    let status_str: String = row.get(6)?;

    Ok(Goal {
        id: GoalId::from(row.get::<_, String>(0)?),
        title: row.get(1)?,
        description: row.get(2)?,
        start_date: row.get(3)?,
        target_date: row.get(4)?,
        priority: Priority::from_db(&priority_str).unwrap_or(Priority::Medium),
        status: Status::from_db(&status_str).unwrap_or(Status::NotStarted),
        estimated_effort_minutes: row.get(7)?,
        created_at: row.get(8)?,
        updated_at: row.get(9)?,
        deleted_at: row.get(10)?,
    })
}