use rusqlite::Connection;
use stark_planning::snapshot::{DateCapacity, GoalMinutes, PlanningSnapshot};
use crate::error::Result;
use crate::{availability_repo, goal_repo, milestone_repo, task_repo};

/// How many days ahead to compute capacity for.
const HORIZON_DAYS: i64 = 180;

pub fn build(conn: &Connection, today: &str) -> Result<PlanningSnapshot> {
    let goals = goal_repo::list(conn)?;

    let mut milestones = Vec::new();
    for g in &goals {
        milestones.extend(milestone_repo::list_for_goal(conn, &g.id)?);
    }

    // All tasks, including completed ones — progress needs them.
    let tasks = task_repo::list(
        conn,
        &stark_domain::TaskFilter {
            goal_id: None,
            milestone_id: None,
            scheduled_date: None,
            include_completed: true,
        },
    )?;

    // Capacity for each date in the horizon.
    let mut capacity_by_date = Vec::with_capacity(HORIZON_DAYS as usize);
    for offset in 0..HORIZON_DAYS {
        let date = add_days(today, offset);
        let weekday = weekday_of(&date);
        let cap = availability_repo::capacity_for_date(conn, &date, weekday)?;
        capacity_by_date.push(DateCapacity {
            date,
            minutes: cap.total_minutes,
        });
    }

    let mut logged_minutes_by_goal = Vec::new();
    for g in &goals {
        let minutes = crate::log_repo::minutes_for_goal(conn, &g.id, "0000-01-01", "9999-12-31")?;
        logged_minutes_by_goal.push(GoalMinutes {
            goal_id: g.id.as_str().to_string(),
            minutes,
        });
    }

    Ok(PlanningSnapshot {
        today: today.to_string(),
        goals,
        milestones,
        tasks,
        capacity_by_date,
        logged_minutes_by_goal,
    })
}

/// Days since 1970-01-01 for a YYYY-MM-DD date.
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// Inverse of `days_from_civil`.
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

fn parse(date: &str) -> (i64, i64, i64) {
    (
        date[0..4].parse().unwrap_or(1970),
        date[5..7].parse().unwrap_or(1),
        date[8..10].parse().unwrap_or(1),
    )
}

fn add_days(date: &str, days: i64) -> String {
    let (y, m, d) = parse(date);
    let (ny, nm, nd) = civil_from_days(days_from_civil(y, m, d) + days);
    format!("{:04}-{:02}-{:02}", ny, nm, nd)
}

/// 0 = Sunday .. 6 = Saturday.
fn weekday_of(date: &str) -> i64 {
    let (y, m, d) = parse(date);
    // 1970-01-01 was a Thursday (weekday 4).
    (days_from_civil(y, m, d) + 4).rem_euclid(7)
}