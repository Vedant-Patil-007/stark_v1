use serde::{Deserialize, Serialize};
use crate::enums::{Priority, Status};
use crate::ids::{CriterionId, GoalId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: GoalId,
    pub title: String,
    pub description: Option<String>,
    /// Local calendar date, YYYY-MM-DD.
    pub start_date: Option<String>,
    /// Local calendar date, YYYY-MM-DD.
    pub target_date: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub estimated_effort_minutes: Option<i64>,
    /// ISO-8601 UTC.
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriterion {
    pub id: CriterionId,
    pub goal_id: GoalId,
    pub text: String,
    pub is_met: bool,
    pub met_at: Option<String>,
    pub order_index: i64,
}

/// Input for creating a goal. Separate from `Goal` because the caller
/// does not supply ids or timestamps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGoal {
    pub title: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub priority: Priority,
    pub estimated_effort_minutes: Option<i64>,
    pub success_criteria: Vec<String>,
}