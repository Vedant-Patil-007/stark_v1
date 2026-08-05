use rusqlite::Connection;
use stark_domain::{GoalId, Milestone, MilestoneId, NewMilestone, Status};
use stark_storage::{goal_repo, milestone_repo};
use crate::error::{CommandError, Result};
use crate::validate;

pub fn create_milestone(conn: &Connection, mut input: NewMilestone) -> Result<Milestone> {
    input.title = validate::title(&input.title)?;
    input.description = validate::optional_description(input.description)?;
    input.target_date = validate::optional_date(input.target_date, "target_date")?;

    // The parent goal must exist. Without this the FK would fail with an
    // opaque SQLite error instead of a clear message.
    if goal_repo::get(conn, &input.goal_id)?.is_none() {
        return Err(CommandError::NotFound(format!("goal {}", input.goal_id)));
    }

    Ok(milestone_repo::create(conn, input)?)
}

pub fn list_milestones(conn: &Connection, goal_id: &GoalId) -> Result<Vec<Milestone>> {
    Ok(milestone_repo::list_for_goal(conn, goal_id)?)
}

pub fn set_milestone_status(
    conn: &Connection,
    id: &MilestoneId,
    status: Status,
) -> Result<()> {
    if milestone_repo::set_status(conn, id, status)? {
        Ok(())
    } else {
        Err(CommandError::NotFound(format!("milestone {id}")))
    }
}

pub fn delete_milestone(conn: &Connection, id: &MilestoneId) -> Result<()> {
    if milestone_repo::soft_delete(conn, id)? {
        Ok(())
    } else {
        Err(CommandError::NotFound(format!("milestone {id}")))
    }
}