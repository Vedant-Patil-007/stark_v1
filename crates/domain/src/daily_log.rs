use serde::{Deserialize, Serialize};
use crate::ids::{GoalId, LogEntryId, MilestoneId, TaskId};

// crate::ids::define_id_pub!(LogEntryId);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: LogEntryId,
    /// Local date, YYYY-MM-DD.
    pub log_date: String,
    pub task_id: Option<TaskId>,
    pub milestone_id: Option<MilestoneId>,
    pub goal_id: Option<GoalId>,
    pub activity: String,
    pub duration_minutes: Option<i64>,
    pub category: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLogEntry {
    pub log_date: String,
    pub task_id: Option<TaskId>,
    pub milestone_id: Option<MilestoneId>,
    pub goal_id: Option<GoalId>,
    pub activity: String,
    pub duration_minutes: Option<i64>,
    pub category: Option<String>,
    pub notes: Option<String>,
}