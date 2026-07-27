use rusqlite::Connection;
use stark_domain::{Goal, GoalId, NewGoal, SuccessCriterion};
use stark_storage::goal_repo;
use crate::error::{CommandError, Result};
use crate::validate;

pub fn create_goal(conn: &mut Connection, mut input: NewGoal) -> Result<Goal> {
    input.title = validate::title(&input.title)?;
    input.description = validate::optional_description(input.description)?;
    input.start_date = validate::optional_date(input.start_date, "start_date")?;
    input.target_date = validate::optional_date(input.target_date, "target_date")?;
    validate::date_order(&input.start_date, &input.target_date)?;

    if let Some(mins) = input.estimated_effort_minutes {
        if mins < 0 {
            return Err(CommandError::Validation(
                "estimated effort cannot be negative".into(),
            ));
        }
    }

    input.success_criteria = input
        .success_criteria
        .into_iter()
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty())
        .collect();

    Ok(goal_repo::create(conn, input)?)
}

pub fn list_goals(conn: &Connection) -> Result<Vec<Goal>> {
    Ok(goal_repo::list(conn)?)
}

pub fn get_goal(conn: &Connection, id: &GoalId) -> Result<Goal> {
    goal_repo::get(conn, id)?
        .ok_or_else(|| CommandError::NotFound(format!("goal {id}")))
}

pub fn goal_criteria(conn: &Connection, id: &GoalId) -> Result<Vec<SuccessCriterion>> {
    Ok(goal_repo::criteria_for(conn, id)?)
}

pub fn delete_goal(conn: &Connection, id: &GoalId) -> Result<()> {
    if goal_repo::soft_delete(conn, id)? {
        Ok(())
    } else {
        Err(CommandError::NotFound(format!("goal {id}")))
    }
}