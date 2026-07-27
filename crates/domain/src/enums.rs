use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
    Cancelled,
}

impl Priority {
    pub fn as_db(&self) -> &'static str {
        match self {
            Priority::Low => "LOW",
            Priority::Medium => "MEDIUM",
            Priority::High => "HIGH",
            Priority::Critical => "CRITICAL",
        }
    }

    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "LOW" => Some(Priority::Low),
            "MEDIUM" => Some(Priority::Medium),
            "HIGH" => Some(Priority::High),
            "CRITICAL" => Some(Priority::Critical),
            _ => None,
        }
    }
}

impl Status {
    pub fn as_db(&self) -> &'static str {
        match self {
            Status::NotStarted => "NOT_STARTED",
            Status::InProgress => "IN_PROGRESS",
            Status::Completed => "COMPLETED",
            Status::Cancelled => "CANCELLED",
        }
    }

    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "NOT_STARTED" => Some(Status::NotStarted),
            "IN_PROGRESS" => Some(Status::InProgress),
            "COMPLETED" => Some(Status::Completed),
            "CANCELLED" => Some(Status::Cancelled),
            _ => None,
        }
    }
}