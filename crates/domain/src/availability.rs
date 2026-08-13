use serde::{Deserialize, Serialize};
use crate::ids::{AvailabilityId, ExceptionId};

/// A recurring weekly availability window.
/// Minutes are measured from local midnight (0..1440).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityWindow {
    pub id: AvailabilityId,
    /// 0 = Sunday .. 6 = Saturday
    pub weekday: i64,
    pub start_minute: i64,
    pub end_minute: i64,
    pub label: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAvailabilityWindow {
    pub weekday: i64,
    pub start_minute: i64,
    pub end_minute: i64,
    pub label: Option<String>,
}

/// A date-specific override. `is_available == false` removes time from the
/// weekly template; `true` adds extra time on that date only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityException {
    pub id: ExceptionId,
    pub date: String,
    pub start_minute: i64,
    pub end_minute: i64,
    pub is_available: bool,
    pub note: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAvailabilityException {
    pub date: String,
    pub start_minute: i64,
    pub end_minute: i64,
    pub is_available: bool,
    pub note: Option<String>,
}

/// A half-open interval [start, end) in minutes from local midnight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interval {
    pub start: i64,
    pub end: i64,
}

impl Interval {
    pub fn minutes(&self) -> i64 {
        (self.end - self.start).max(0)
    }
}

/// Computed capacity for a single date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayCapacity {
    pub date: String,
    pub windows: Vec<Interval>,
    pub total_minutes: i64,
}