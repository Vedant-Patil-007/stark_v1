use rusqlite::Connection;
use stark_domain::{NewTask, Status, Task, TaskFilter, TaskId};
use stark_storage::{goal_repo, milestone_repo, task_repo};
use crate::error::{CommandError, Result};
use crate::validate;


pub fn create_task(conn: &Connection, mut input: NewTask) -> Result<Task> {
    input.title = validate::title(&input.title)?;
    input.description = validate::optional_description(input.description)?;
    input.due_date = validate::optional_date(input.due_date, "due_date")?;
    input.scheduled_date = validate::optional_date(input.scheduled_date, "scheduled_date")?;

    if let Some(mins) = input.estimated_minutes {
        if mins <= 0 {
            return Err(CommandError::Validation(
                "estimated minutes must be positive".into(),
            ));
        }
        if mins > 24 * 60 {
            return Err(CommandError::Validation(
                "a single task cannot exceed 24 hours; split it into smaller tasks".into(),
            ));
        }
    }

    if let Some(goal_id) = &input.goal_id {
        if goal_repo::get(conn, goal_id)?.is_none() {
            return Err(CommandError::NotFound(format!("goal {goal_id}")));
        }
    }

    // A milestone implies its goal. Verify consistency rather than guessing.
    if let Some(milestone_id) = &input.milestone_id {
        match milestone_repo::get(conn, milestone_id)? {
            None => {
                return Err(CommandError::NotFound(format!("milestone {milestone_id}")));
            }
            Some(m) => match &input.goal_id {
                Some(g) if g != &m.goal_id => {
                    return Err(CommandError::Validation(
                        "milestone does not belong to the specified goal".into(),
                    ));
                }
                // Milestone given without a goal: inherit the milestone's goal.
                None => input.goal_id = Some(m.goal_id.clone()),
                _ => {}
            },
        }
    }

    Ok(task_repo::create(conn, input)?)
}

pub fn list_tasks(conn: &Connection, filter: &TaskFilter) -> Result<Vec<Task>> {
    Ok(task_repo::list(conn, filter)?)
}

pub fn set_task_status(conn: &Connection, id: &TaskId, status: Status) -> Result<()> {
    if task_repo::set_status(conn, id, status)? {
        Ok(())
    } else {
        Err(CommandError::NotFound(format!("task {id}")))
    }
}

pub fn reschedule_task(
    conn: &Connection,
    id: &TaskId,
    scheduled_date: Option<String>,
) -> Result<()> {
    let date = validate::optional_date(scheduled_date, "scheduled_date")?;
    if task_repo::set_scheduled_date(conn, id, date)? {
        Ok(())
    } else {
        Err(CommandError::NotFound(format!("task {id}")))
    }
}

pub fn delete_task(conn: &Connection, id: &TaskId) -> Result<()> {
    if task_repo::soft_delete(conn, id)? {
        Ok(())
    } else {
        Err(CommandError::NotFound(format!("task {id}")))
    }
}


/// All non-deleted tasks with a scheduled or due date inside the range.
pub fn tasks_in_range(conn: &Connection, from: &str, to: &str) -> Result<Vec<Task>> {
    if from > to {
        return Err(CommandError::Validation(
            "range start cannot be after range end".into(),
        ));
    }
    Ok(stark_storage::task_repo::list_in_range(conn, from, to)?)
}