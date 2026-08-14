use serde::{Deserialize, Serialize};
use stark_domain::{Goal, Milestone, Task};

/// Everything the engine needs, assembled by the storage layer.
/// The engine never queries; it only reads this struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningSnapshot {
    /// Today, as a local YYYY-MM-DD date.
    pub today: String,
    pub goals: Vec<Goal>,
    pub milestones: Vec<Milestone>,
    /// All non-deleted tasks, completed and outstanding.
    pub tasks: Vec<Task>,
    /// Available minutes per date, for dates from today forward.
    pub capacity_by_date: Vec<DateCapacity>,
    /// Minutes already logged per goal, all time.
    pub logged_minutes_by_goal: Vec<GoalMinutes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateCapacity {
    pub date: String,
    pub minutes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalMinutes {
    pub goal_id: String,
    pub minutes: i64,
}