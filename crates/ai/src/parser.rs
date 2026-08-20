use regex::Regex;
use stark_domain::Priority;
use crate::action::AiAction;

/// Parse a command with no model. Returns None if nothing matches,
/// in which case the caller escalates to a provider.
///
/// `today` is a local YYYY-MM-DD date, used to resolve relative dates.
pub fn parse(input: &str, today: &str) -> Option<AiAction> {
    let s = input.trim();
    if s.is_empty() {
        return None;
    }

    try_complete(s)
        .or_else(|| try_log(s, today))
        .or_else(|| try_reschedule(s, today))
        .or_else(|| try_add_task(s, today))
        .or_else(|| try_query(s))
}

// ---------- "done X" / "complete X" / "finished X" ----------

fn try_complete(s: &str) -> Option<AiAction> {
    let re = Regex::new(r"(?i)^(?:done|complete|completed|finish|finished)\s+(.+)$").ok()?;
    let caps = re.captures(s)?;
    let task_ref = caps.get(1)?.as_str().trim().to_string();
    if task_ref.is_empty() {
        return None;
    }
    Some(AiAction::CompleteTask { task_ref })
}

// ---------- "log 2h X" / "log 90m X on GOAL" ----------

fn try_log(s: &str, today: &str) -> Option<AiAction> {
    let re = Regex::new(
        r"(?i)^log\s+(?:(\d+)\s*h(?:ours?|rs?)?)?\s*(?:(\d+)\s*m(?:in(?:ute)?s?)?)?\s+(.+)$",
    )
    .ok()?;
    let caps = re.captures(s)?;

    let hours: i64 = caps.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
    let mins: i64 = caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
    let total = hours * 60 + mins;

    let rest = caps.get(3)?.as_str().trim();
    if rest.is_empty() {
        return None;
    }

    // Split a trailing " on <goal>" if present.
    let (activity, goal_ref) = split_on_keyword(rest, " on ");

    Some(AiAction::LogWork {
        activity: activity.to_string(),
        duration_minutes: if total > 0 { Some(total) } else { None },
        goal_ref,
        log_date: Some(today.to_string()),
    })
}

// ---------- "move X to friday" / "reschedule X to 2026-09-01" ----------

fn try_reschedule(s: &str, today: &str) -> Option<AiAction> {
    let re = Regex::new(r"(?i)^(?:move|reschedule|shift)\s+(.+?)\s+to\s+(.+)$").ok()?;
    let caps = re.captures(s)?;
    let task_ref = caps.get(1)?.as_str().trim().to_string();
    let date_str = caps.get(2)?.as_str().trim();
    let scheduled_date = resolve_date(date_str, today)?;

    Some(AiAction::RescheduleTask {
        task_ref,
        scheduled_date,
    })
}

// ---------- "add task X" / "add task X by friday" ----------

fn try_add_task(s: &str, today: &str) -> Option<AiAction> {
    let re = Regex::new(r"(?i)^(?:add|create|new)\s+task\s+(.+)$").ok()?;
    let caps = re.captures(s)?;
    let mut rest = caps.get(1)?.as_str().trim().to_string();

    // Optional trailing "by <date>" sets the due date.
    let mut due_date = None;
    if let Some(idx) = find_keyword(&rest, " by ") {
        let (title, date_part) = rest.split_at(idx);
        let date_part = date_part[4..].trim();
        if let Some(d) = resolve_date(date_part, today) {
            due_date = Some(d);
            rest = title.trim().to_string();
        }
    }

    // Optional trailing "for <goal>".
    let (title, goal_ref) = split_on_keyword(&rest, " for ");

    if title.is_empty() {
        return None;
    }

    Some(AiAction::CreateTask {
        title: title.to_string(),
        goal_ref,
        due_date,
        scheduled_date: None,
        estimated_minutes: None,
        priority: None::<Priority>,
    })
}

// ---------- "how am I doing" / "progress" ----------

fn try_query(s: &str) -> Option<AiAction> {
    let re = Regex::new(
        r"(?i)^(?:how am i doing|progress|status|am i on track)(?:\s+on\s+(.+))?$",
    )
    .ok()?;
    let caps = re.captures(s)?;
    Some(AiAction::QueryProgress {
        goal_ref: caps.get(1).map(|m| m.as_str().trim().to_string()),
    })
}

// ---------- helpers ----------

fn find_keyword(s: &str, kw: &str) -> Option<usize> {
    s.to_lowercase().rfind(kw)
}

/// Split "activity on goal" into ("activity", Some("goal")).
fn split_on_keyword<'a>(s: &'a str, kw: &str) -> (&'a str, Option<String>) {
    match find_keyword(s, kw) {
        Some(idx) => {
            let (left, right) = s.split_at(idx);
            let right = right[kw.len()..].trim();
            if right.is_empty() {
                (s.trim(), None)
            } else {
                (left.trim(), Some(right.to_string()))
            }
        }
        None => (s.trim(), None),
    }
}

/// Resolve "today", "tomorrow", a weekday name, or a literal YYYY-MM-DD.
pub fn resolve_date(input: &str, today: &str) -> Option<String> {
    let s = input.trim().to_lowercase();

    if is_iso_date(&s) {
        return Some(s);
    }

    match s.as_str() {
        "today" => return Some(today.to_string()),
        "tomorrow" => return Some(add_days(today, 1)),
        "yesterday" => return Some(add_days(today, -1)),
        _ => {}
    }

    // Weekday name: the next occurrence, strictly after today.
    let target = weekday_index(&s)?;
    let current = weekday_of(today);
    let mut delta = (target - current + 7) % 7;
    if delta == 0 {
        delta = 7;
    }
    Some(add_days(today, delta))
}

fn is_iso_date(s: &str) -> bool {
    s.len() == 10
        && s.as_bytes()[4] == b'-'
        && s.as_bytes()[7] == b'-'
        && s.chars().enumerate().all(|(i, c)| {
            if i == 4 || i == 7 { c == '-' } else { c.is_ascii_digit() }
        })
}

fn weekday_index(s: &str) -> Option<i64> {
    Some(match s {
        "sunday" | "sun" => 0,
        "monday" | "mon" => 1,
        "tuesday" | "tue" | "tues" => 2,
        "wednesday" | "wed" => 3,
        "thursday" | "thu" | "thurs" => 4,
        "friday" | "fri" => 5,
        "saturday" | "sat" => 6,
        _ => return None,
    })
}

fn parse_ymd(s: &str) -> (i64, i64, i64) {
    (
        s[0..4].parse().unwrap_or(1970),
        s[5..7].parse().unwrap_or(1),
        s[8..10].parse().unwrap_or(1),
    )
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn add_days(date: &str, days: i64) -> String {
    let (y, m, d) = parse_ymd(date);
    let (ny, nm, nd) = civil_from_days(days_from_civil(y, m, d) + days);
    format!("{:04}-{:02}-{:02}", ny, nm, nd)
}

fn weekday_of(date: &str) -> i64 {
    let (y, m, d) = parse_ymd(date);
    (days_from_civil(y, m, d) + 4).rem_euclid(7)
}