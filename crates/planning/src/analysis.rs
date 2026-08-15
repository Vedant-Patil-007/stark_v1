use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GoalHealth {
    OnTrack,
    AtRisk,
    Behind,
    Critical,
    /// No deadline, or nothing left to do — risk is not meaningful.
    NotApplicable,
}

/// How much of the analysis rests on real data rather than gaps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalAnalysis {
    pub goal_id: String,
    pub title: String,

    /// Effort-weighted completion, 0.0..1.0.
    pub progress: f64,
    pub tasks_total: usize,
    pub tasks_completed: usize,

    /// Estimated minutes still outstanding.
    pub workload_remaining_minutes: i64,
    /// Capacity between today and the deadline, shared across all goals.
    pub capacity_available_minutes: i64,
    /// Positive means short of time.
    pub shortfall_minutes: i64,

    pub days_remaining: Option<i64>,
    pub health: GoalHealth,

    /// Fraction of outstanding tasks that carry an estimate, 0.0..1.0.
    pub estimate_coverage: f64,
    pub unestimated_task_count: usize,
    pub confidence: Confidence,

    /// Human-readable explanation of the health verdict.
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub generated_for: String,
    pub goals: Vec<GoalAnalysis>,
    pub today_task_count: usize,
    pub today_planned_minutes: i64,
    pub today_capacity_minutes: i64,
    pub overdue_task_count: usize,
    pub upcoming: Vec<UpcomingItem>,
    pub capacity_next_7_days_minutes: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpcomingItem {
    pub date: String,
    pub label: String,
    pub kind: UpcomingKind,
    pub days_away: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpcomingKind {
    TaskDue,
    MilestoneTarget,
    GoalTarget,
}