use serde::{Deserialize, Serialize};
use stark_domain::Priority;

/// The complete set of things an AI may propose. Anything outside this
/// enum fails to deserialize and is rejected before reaching the validator.
///
/// Note that every reference to an existing entity is a NAME, never an ID.
/// The application resolves names by lookup; a model that emitted IDs would
/// hallucinate well-formed UUIDs pointing at nothing, or at the wrong thing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum AiAction {
    CreateTask {
        title: String,
        #[serde(default)]
        goal_ref: Option<String>,
        #[serde(default)]
        due_date: Option<String>,
        #[serde(default)]
        scheduled_date: Option<String>,
        #[serde(default)]
        estimated_minutes: Option<i64>,
        #[serde(default)]
        priority: Option<Priority>,
    },

    CompleteTask {
        task_ref: String,
    },

    RescheduleTask {
        task_ref: String,
        scheduled_date: String,
    },

    CreateGoal {
        title: String,
        #[serde(default)]
        target_date: Option<String>,
        #[serde(default)]
        priority: Option<Priority>,
    },

    LogWork {
        activity: String,
        #[serde(default)]
        duration_minutes: Option<i64>,
        #[serde(default)]
        goal_ref: Option<String>,
        #[serde(default)]
        log_date: Option<String>,
    },

    SetAvailability {
        date: String,
        start_minute: i64,
        end_minute: i64,
        is_available: bool,
    },

    QueryProgress {
        #[serde(default)]
        goal_ref: Option<String>,
    },

    /// Not enough information to act. The spec's rule: unknown stays unknown.
    CreateInboxItem {
        raw_text: String,
    },

    /// The request was understood but is ambiguous. Ask, never guess.
    NeedsClarification {
        question: String,
        #[serde(default)]
        candidates: Vec<String>,
    },
}

impl AiAction {
    /// Whether this action needs explicit user confirmation before executing.
    pub fn is_high_impact(&self) -> bool {
        matches!(
            self,
            AiAction::SetAvailability { .. } | AiAction::CreateGoal { .. }
        )
    }

    pub fn name(&self) -> &'static str {
        match self {
            AiAction::CreateTask { .. } => "CREATE_TASK",
            AiAction::CompleteTask { .. } => "COMPLETE_TASK",
            AiAction::RescheduleTask { .. } => "RESCHEDULE_TASK",
            AiAction::CreateGoal { .. } => "CREATE_GOAL",
            AiAction::LogWork { .. } => "LOG_WORK",
            AiAction::SetAvailability { .. } => "SET_AVAILABILITY",
            AiAction::QueryProgress { .. } => "QUERY_PROGRESS",
            AiAction::CreateInboxItem { .. } => "CREATE_INBOX_ITEM",
            AiAction::NeedsClarification { .. } => "NEEDS_CLARIFICATION",
        }
    }
}