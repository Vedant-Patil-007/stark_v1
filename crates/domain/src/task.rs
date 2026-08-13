use serde::{Deserialize, Serialize};
use crate::enums::{Priority, Status};
use crate::ids::{GoalId, MilestoneId, TaskId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub goal_id: Option<GoalId>,
    pub milestone_id: Option<MilestoneId>,
    pub title: String,
    pub description: Option<String>,
    /// When it MUST be done by. Local date, YYYY-MM-DD.
    pub due_date: Option<String>,
    /// When I INTEND to do it. Local date, YYYY-MM-DD.
    pub scheduled_date: Option<String>,
    pub estimated_minutes: Option<i64>,
    pub priority: Priority,
    pub status: Status,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTask {
    pub goal_id: Option<GoalId>,
    pub milestone_id: Option<MilestoneId>,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub scheduled_date: Option<String>,
    pub estimated_minutes: Option<i64>,
    pub priority: Priority,
}

/// Filter for listing tasks. All fields optional; None means "no constraint".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskFilter {
    pub goal_id: Option<GoalId>,
    pub milestone_id: Option<MilestoneId>,
    pub scheduled_date: Option<String>,
    pub include_completed: bool,
}