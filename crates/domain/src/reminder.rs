use serde::{Deserialize, Serialize};
use crate::ids::{GoalId, ReminderId, TaskId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReminderStatus {
    Pending,
    Fired,
    /// Its time passed while the app was closed.
    Missed,
    Dismissed,
}

impl ReminderStatus {
    pub fn as_db(&self) -> &'static str {
        match self {
            ReminderStatus::Pending => "PENDING",
            ReminderStatus::Fired => "FIRED",
            ReminderStatus::Missed => "MISSED",
            ReminderStatus::Dismissed => "DISMISSED",
        }
    }

    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "PENDING" => Some(ReminderStatus::Pending),
            "FIRED" => Some(ReminderStatus::Fired),
            "MISSED" => Some(ReminderStatus::Missed),
            "DISMISSED" => Some(ReminderStatus::Dismissed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: ReminderId,
    pub task_id: Option<TaskId>,
    pub goal_id: Option<GoalId>,
    /// ISO-8601 UTC instant.
    pub fire_at_utc: String,
    pub title: String,
    pub body: Option<String>,
    pub status: ReminderStatus,
    pub fired_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewReminder {
    pub task_id: Option<TaskId>,
    pub goal_id: Option<GoalId>,
    pub fire_at_utc: String,
    pub title: String,
    pub body: Option<String>,
}