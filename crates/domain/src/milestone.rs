use serde::{Deserialize, Serialize};
use crate::enums::Status;
use crate::ids::{GoalId, MilestoneId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: MilestoneId,
    pub goal_id: GoalId,
    pub title: String,
    pub description: Option<String>,
    pub target_date: Option<String>,
    pub status: Status,
    pub order_index: i64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMilestone {
    pub goal_id: GoalId,
    pub title: String,
    pub description: Option<String>,
    pub target_date: Option<String>,
}