use stark_domain::{Goal, Status, Task};
use crate::analysis::{Analysis, Confidence, GoalAnalysis, GoalHealth, UpcomingItem, UpcomingKind};
use crate::snapshot::PlanningSnapshot;

/// Risk thresholds, expressed as shortfall relative to remaining workload.
/// Tune these in one place once real numbers are available.
pub const AT_RISK_THRESHOLD: f64 = 0.0;   // any shortfall at all
pub const BEHIND_THRESHOLD: f64 = 0.20;   // short by more than 20%
pub const CRITICAL_THRESHOLD: f64 = 0.50; // short by more than 50%

/// Below this share of estimated tasks, the analysis is low confidence.
pub const LOW_COVERAGE: f64 = 0.60;
pub const HIGH_COVERAGE: f64 = 0.90;



pub fn analyze(snapshot: &PlanningSnapshot) -> Analysis {
    let goals: Vec<GoalAnalysis> = snapshot
        .goals
        .iter()
        .filter(|g| g.deleted_at.is_none())
        .filter(|g| g.status != Status::Cancelled)
        .map(|g| analyze_goal(g, snapshot))
        .collect();

    let today_tasks: Vec<&Task> = snapshot
        .tasks
        .iter()
        .filter(|t| t.deleted_at.is_none())
        .filter(|t| t.scheduled_date.as_deref() == Some(snapshot.today.as_str()))
        .filter(|t| !is_finished(t))
        .collect();

    let today_planned_minutes = today_tasks
        .iter()
        .filter_map(|t| t.estimated_minutes)
        .sum();

    let today_capacity_minutes = snapshot
        .capacity_by_date
        .iter()
        .find(|c| c.date == snapshot.today)
        .map(|c| c.minutes)
        .unwrap_or(0);

    let overdue_task_count = snapshot
        .tasks
        .iter()
        .filter(|t| t.deleted_at.is_none())
        .filter(|t| !is_finished(t))
        .filter(|t| match &t.due_date {
            Some(d) => d.as_str() < snapshot.today.as_str(),
            None => false,
        })
        .count();
// --- Upcoming: deadlines within the next 30 days ---
    let mut upcoming: Vec<UpcomingItem> = Vec::new();

    for t in snapshot.tasks.iter() {
        if t.deleted_at.is_some() || is_finished(t) {
            continue;
        }
        if let Some(due) = &t.due_date {
            let days = days_between(&snapshot.today, due);
            if (0..=30).contains(&days) {
                upcoming.push(UpcomingItem {
                    date: due.clone(),
                    label: t.title.clone(),
                    kind: UpcomingKind::TaskDue,
                    days_away: days,
                });
            }
        }
    }

    for m in snapshot.milestones.iter() {
        if m.deleted_at.is_some() || m.status == Status::Completed {
            continue;
        }
        if let Some(target) = &m.target_date {
            let days = days_between(&snapshot.today, target);
            if (0..=30).contains(&days) {
                upcoming.push(UpcomingItem {
                    date: target.clone(),
                    label: m.title.clone(),
                    kind: UpcomingKind::MilestoneTarget,
                    days_away: days,
                });
            }
        }
    }

    for g in snapshot.goals.iter() {
        if g.deleted_at.is_some() || g.status == Status::Completed {
            continue;
        }
        if let Some(target) = &g.target_date {
            let days = days_between(&snapshot.today, target);
            if (0..=30).contains(&days) {
                upcoming.push(UpcomingItem {
                    date: target.clone(),
                    label: g.title.clone(),
                    kind: UpcomingKind::GoalTarget,
                    days_away: days,
                });
            }
        }
    }

    upcoming.sort_by(|a, b| a.date.cmp(&b.date));
    upcoming.truncate(8);

    let capacity_next_7_days_minutes: i64 = snapshot
        .capacity_by_date
        .iter()
        .filter(|c| c.date.as_str() >= snapshot.today.as_str())
        .take(7)
        .map(|c| c.minutes)
        .sum();
   Analysis {
        generated_for: snapshot.today.clone(),
        goals,
        today_task_count: today_tasks.len(),
        today_planned_minutes,
        today_capacity_minutes,
        overdue_task_count,
        upcoming,
        capacity_next_7_days_minutes,
    }
}

fn is_finished(t: &Task) -> bool {
    matches!(t.status, Status::Completed | Status::Cancelled)
}

fn analyze_goal(goal: &Goal, snapshot: &PlanningSnapshot) -> GoalAnalysis {
    let tasks: Vec<&Task> = snapshot
        .tasks
        .iter()
        .filter(|t| t.deleted_at.is_none())
        .filter(|t| t.goal_id.as_ref().map(|g| g.as_str()) == Some(goal.id.as_str()))
        .filter(|t| t.status != Status::Cancelled)
        .collect();

    let tasks_total = tasks.len();
    let tasks_completed = tasks
        .iter()
        .filter(|t| t.status == Status::Completed)
        .count();

    let outstanding: Vec<&&Task> = tasks.iter().filter(|t| !is_finished(t)).collect();

    // --- Progress: effort-weighted where estimates exist, else task count ---
    let total_estimated: i64 = tasks.iter().filter_map(|t| t.estimated_minutes).sum();
    let done_estimated: i64 = tasks
        .iter()
        .filter(|t| t.status == Status::Completed)
        .filter_map(|t| t.estimated_minutes)
        .sum();

    let estimated_task_count = tasks.iter().filter(|t| t.estimated_minutes.is_some()).count();
    let coverage = if tasks_total == 0 {
        1.0
    } else {
        estimated_task_count as f64 / tasks_total as f64
    };

    let progress = if total_estimated > 0 && coverage >= LOW_COVERAGE {
        done_estimated as f64 / total_estimated as f64
    } else if tasks_total > 0 {
        tasks_completed as f64 / tasks_total as f64
    } else {
        0.0
    };

    // --- Workload remaining ---
    let workload_remaining_minutes: i64 = outstanding
        .iter()
        .filter_map(|t| t.estimated_minutes)
        .sum();

    let unestimated_task_count = outstanding
        .iter()
        .filter(|t| t.estimated_minutes.is_none())
        .count();

    // --- Days and capacity until the deadline ---
    let days_remaining = goal
        .target_date
        .as_ref()
        .map(|target| days_between(&snapshot.today, target));

    let capacity_available_minutes = match &goal.target_date {
        Some(target) => snapshot
            .capacity_by_date
            .iter()
            .filter(|c| c.date.as_str() >= snapshot.today.as_str())
            .filter(|c| c.date.as_str() <= target.as_str())
            .map(|c| c.minutes)
            .sum(),
        None => 0,
    };

    let shortfall_minutes = workload_remaining_minutes - capacity_available_minutes;

    // --- Confidence ---
    let confidence = if coverage >= HIGH_COVERAGE {
        Confidence::High
    } else if coverage >= LOW_COVERAGE {
        Confidence::Medium
    } else {
        Confidence::Low
    };

    // --- Health ---
    let (health, reason) = classify(
        goal,
        days_remaining,
        workload_remaining_minutes,
        capacity_available_minutes,
        shortfall_minutes,
        outstanding.len(),
    );

    GoalAnalysis {
        goal_id: goal.id.as_str().to_string(),
        title: goal.title.clone(),
        progress,
        tasks_total,
        tasks_completed,
        workload_remaining_minutes,
        capacity_available_minutes,
        shortfall_minutes,
        days_remaining,
        health,
        estimate_coverage: coverage,
        unestimated_task_count,
        confidence,
        reason,
    }
}

fn classify(
    goal: &Goal,
    days_remaining: Option<i64>,
    workload: i64,
    capacity: i64,
    shortfall: i64,
    outstanding_count: usize,
) -> (GoalHealth, String) {
    if goal.status == Status::Completed {
        return (GoalHealth::NotApplicable, "Goal is complete.".into());
    }

    let Some(days) = days_remaining else {
        return (
            GoalHealth::NotApplicable,
            "No target date set, so deadline risk cannot be calculated.".into(),
        );
    };

    if outstanding_count == 0 {
        return (
            GoalHealth::NotApplicable,
            "No outstanding tasks.".into(),
        );
    }

    if days < 0 {
        return (
            GoalHealth::Critical,
            format!(
                "Deadline passed {} day(s) ago with {} task(s) outstanding.",
                -days, outstanding_count
            ),
        );
    }

    if workload == 0 {
        return (
            GoalHealth::NotApplicable,
            format!(
                "{} outstanding task(s), but none have time estimates.",
                outstanding_count
            ),
        );
    }

    if shortfall <= 0 {
        return (
            GoalHealth::OnTrack,
            format!(
                "{} of work remaining, {} available before the deadline.",
                fmt_minutes(workload),
                fmt_minutes(capacity)
            ),
        );
    }

    let ratio = shortfall as f64 / workload as f64;
    let msg = format!(
        "{} of work remaining but only {} available — short by {}.",
        fmt_minutes(workload),
        fmt_minutes(capacity),
        fmt_minutes(shortfall)
    );

    if ratio > CRITICAL_THRESHOLD {
        (GoalHealth::Critical, msg)
    } else if ratio > BEHIND_THRESHOLD {
        (GoalHealth::Behind, msg)
    } else {
        (GoalHealth::AtRisk, msg)
    }
}

pub fn fmt_minutes(m: i64) -> String {
    let sign = if m < 0 { "-" } else { "" };
    let m = m.abs();
    let h = m / 60;
    let min = m % 60;
    if h == 0 {
        format!("{sign}{min}m")
    } else if min == 0 {
        format!("{sign}{h}h")
    } else {
        format!("{sign}{h}h {min}m")
    }
}

/// Whole days from `from` to `to`, both YYYY-MM-DD. Negative if `to` is past.
pub fn days_between(from: &str, to: &str) -> i64 {
    match (parse_ymd(from), parse_ymd(to)) {
        (Some(a), Some(b)) => days_from_civil(b) - days_from_civil(a),
        _ => 0,
    }
}

fn parse_ymd(s: &str) -> Option<(i64, i64, i64)> {
    let b = s.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    let y = s[0..4].parse().ok()?;
    let m = s[5..7].parse().ok()?;
    let d = s[8..10].parse().ok()?;
    Some((y, m, d))
}

/// Days since 1970-01-01. Howard Hinnant's civil-date algorithm.
fn days_from_civil((y, m, d): (i64, i64, i64)) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}