use stark_domain::{Goal, GoalId, Priority, Status, Task, TaskId};
use stark_planning::engine::{analyze, days_between, fmt_minutes};
use stark_planning::snapshot::{DateCapacity, PlanningSnapshot};
use stark_planning::GoalHealth;

fn goal(id: &str, target: Option<&str>) -> Goal {
    Goal {
        id: GoalId::from(id.to_string()),
        title: format!("Goal {id}"),
        description: None,
        start_date: None,
        target_date: target.map(|s| s.to_string()),
        priority: Priority::Medium,
        status: Status::NotStarted,
        estimated_effort_minutes: None,
        created_at: "2026-01-01T00:00:00Z".into(),
        updated_at: "2026-01-01T00:00:00Z".into(),
        deleted_at: None,
    }
}

fn task(id: &str, goal_id: &str, minutes: Option<i64>, status: Status) -> Task {
    Task {
        id: TaskId::from(id.to_string()),
        goal_id: Some(GoalId::from(goal_id.to_string())),
        milestone_id: None,
        title: format!("Task {id}"),
        description: None,
        due_date: None,
        scheduled_date: None,
        estimated_minutes: minutes,
        priority: Priority::Medium,
        status,
        created_at: "2026-01-01T00:00:00Z".into(),
        updated_at: "2026-01-01T00:00:00Z".into(),
        completed_at: None,
        deleted_at: None,
    }
}

/// `minutes_per_day` of capacity on each of `days` consecutive dates from `start`.
fn capacity(start: &str, days: i64, minutes_per_day: i64) -> Vec<DateCapacity> {
    let mut out = Vec::new();
    let (y, m, d) = (
        start[0..4].parse::<i64>().unwrap(),
        start[5..7].parse::<i64>().unwrap(),
        start[8..10].parse::<i64>().unwrap(),
    );
    // Naive walk; adequate for tests within a single month.
    for i in 0..days {
        out.push(DateCapacity {
            date: format!("{:04}-{:02}-{:02}", y, m, d + i),
            minutes: minutes_per_day,
        });
    }
    out
}

fn snapshot(goals: Vec<Goal>, tasks: Vec<Task>, cap: Vec<DateCapacity>) -> PlanningSnapshot {
    PlanningSnapshot {
        today: "2026-08-01".into(),
        goals,
        milestones: Vec::new(),
        tasks,
        capacity_by_date: cap,
        logged_minutes_by_goal: Vec::new(),
    }
}

// ---------- date arithmetic ----------

#[test]
fn days_between_same_day_is_zero() {
    assert_eq!(days_between("2026-08-01", "2026-08-01"), 0);
}

#[test]
fn days_between_counts_forward() {
    assert_eq!(days_between("2026-08-01", "2026-08-11"), 10);
}

#[test]
fn days_between_is_negative_in_the_past() {
    assert_eq!(days_between("2026-08-11", "2026-08-01"), -10);
}

#[test]
fn days_between_crosses_month_boundary() {
    assert_eq!(days_between("2026-08-30", "2026-09-02"), 3);
}

#[test]
fn days_between_handles_leap_year() {
    assert_eq!(days_between("2024-02-28", "2024-03-01"), 2);
}

#[test]
fn days_between_handles_non_leap_year() {
    assert_eq!(days_between("2026-02-28", "2026-03-01"), 1);
}

// ---------- formatting ----------

#[test]
fn formats_minutes_readably() {
    assert_eq!(fmt_minutes(45), "45m");
    assert_eq!(fmt_minutes(120), "2h");
    assert_eq!(fmt_minutes(150), "2h 30m");
    assert_eq!(fmt_minutes(-90), "-1h 30m");
}

// ---------- health classification ----------

#[test]
fn on_track_when_capacity_exceeds_workload() {
    // 4h of work, 10 days at 2h/day = 20h available
    let s = snapshot(
        vec![goal("g1", Some("2026-08-10"))],
        vec![task("t1", "g1", Some(240), Status::NotStarted)],
        capacity("2026-08-01", 10, 120),
    );
    let a = analyze(&s);
    assert_eq!(a.goals[0].health, GoalHealth::OnTrack);
    assert!(a.goals[0].shortfall_minutes < 0);
}

#[test]
fn at_risk_when_slightly_short() {
    // 10h of work, 5 days at 2h/day = 10h... make it 9h to be 10% short
    let s = snapshot(
        vec![goal("g1", Some("2026-08-05"))],
        vec![task("t1", "g1", Some(600), Status::NotStarted)],
        capacity("2026-08-01", 5, 108), // 5 * 108 = 540 = 9h
    );
    let a = analyze(&s);
    assert_eq!(a.goals[0].health, GoalHealth::AtRisk);
    assert_eq!(a.goals[0].shortfall_minutes, 60);
}

#[test]
fn behind_when_short_by_more_than_twenty_percent() {
    // 10h of work, 7h available -> 30% short
    let s = snapshot(
        vec![goal("g1", Some("2026-08-05"))],
        vec![task("t1", "g1", Some(600), Status::NotStarted)],
        capacity("2026-08-01", 5, 84), // 420 = 7h
    );
    let a = analyze(&s);
    assert_eq!(a.goals[0].health, GoalHealth::Behind);
}

#[test]
fn critical_when_short_by_more_than_half() {
    // 10h of work, 4h available -> 60% short
    let s = snapshot(
        vec![goal("g1", Some("2026-08-05"))],
        vec![task("t1", "g1", Some(600), Status::NotStarted)],
        capacity("2026-08-01", 5, 48), // 240 = 4h
    );
    let a = analyze(&s);
    assert_eq!(a.goals[0].health, GoalHealth::Critical);
}

#[test]
fn critical_when_deadline_has_passed_with_work_outstanding() {
    let s = snapshot(
        vec![goal("g1", Some("2026-07-01"))], // before today
        vec![task("t1", "g1", Some(600), Status::NotStarted)],
        capacity("2026-08-01", 10, 120),
    );
    let a = analyze(&s);
    assert_eq!(a.goals[0].health, GoalHealth::Critical);
    assert!(a.goals[0].days_remaining.unwrap() < 0);
}

#[test]
fn not_applicable_without_a_target_date() {
    let s = snapshot(
        vec![goal("g1", None)],
        vec![task("t1", "g1", Some(600), Status::NotStarted)],
        capacity("2026-08-01", 10, 120),
    );
    assert_eq!(analyze(&s).goals[0].health, GoalHealth::NotApplicable);
}

#[test]
fn not_applicable_when_all_tasks_are_done() {
    let s = snapshot(
        vec![goal("g1", Some("2026-08-10"))],
        vec![task("t1", "g1", Some(600), Status::Completed)],
        capacity("2026-08-01", 10, 120),
    );
    assert_eq!(analyze(&s).goals[0].health, GoalHealth::NotApplicable);
}

// ---------- progress ----------

#[test]
fn progress_is_effort_weighted() {
    // 60m done out of 240m total = 25%
    let s = snapshot(
        vec![goal("g1", Some("2026-08-10"))],
        vec![
            task("t1", "g1", Some(60), Status::Completed),
            task("t2", "g1", Some(180), Status::NotStarted),
        ],
        capacity("2026-08-01", 10, 120),
    );
    let a = analyze(&s);
    assert!((a.goals[0].progress - 0.25).abs() < 0.001);
}

#[test]
fn progress_falls_back_to_task_count_without_estimates() {
    // 1 of 2 done, no estimates -> 50%
    let s = snapshot(
        vec![goal("g1", Some("2026-08-10"))],
        vec![
            task("t1", "g1", None, Status::Completed),
            task("t2", "g1", None, Status::NotStarted),
        ],
        capacity("2026-08-01", 10, 120),
    );
    let a = analyze(&s);
    assert!((a.goals[0].progress - 0.5).abs() < 0.001);
    assert_eq!(a.goals[0].unestimated_task_count, 1);
}

#[test]
fn cancelled_tasks_are_excluded_from_progress() {
    let s = snapshot(
        vec![goal("g1", Some("2026-08-10"))],
        vec![
            task("t1", "g1", Some(60), Status::Completed),
            task("t2", "g1", Some(60), Status::Cancelled),
        ],
        capacity("2026-08-01", 10, 120),
    );
    let a = analyze(&s);
    assert_eq!(a.goals[0].tasks_total, 1);
    assert!((a.goals[0].progress - 1.0).abs() < 0.001);
}

// ---------- confidence ----------

#[test]
fn confidence_is_low_when_estimates_are_sparse() {
    use stark_planning::Confidence;
    let s = snapshot(
        vec![goal("g1", Some("2026-08-10"))],
        vec![
            task("t1", "g1", Some(60), Status::NotStarted),
            task("t2", "g1", None, Status::NotStarted),
            task("t3", "g1", None, Status::NotStarted),
        ],
        capacity("2026-08-01", 10, 120),
    );
    let a = analyze(&s);
    assert_eq!(a.goals[0].confidence, Confidence::Low);
    assert_eq!(a.goals[0].unestimated_task_count, 2);
}

// ---------- day-level figures ----------

#[test]
fn counts_overdue_tasks() {
    let mut t = task("t1", "g1", Some(60), Status::NotStarted);
    t.due_date = Some("2026-07-20".into()); // before today
    let s = snapshot(vec![goal("g1", Some("2026-08-10"))], vec![t], capacity("2026-08-01", 10, 120));
    assert_eq!(analyze(&s).overdue_task_count, 1);
}

#[test]
fn completed_tasks_are_never_overdue() {
    let mut t = task("t1", "g1", Some(60), Status::Completed);
    t.due_date = Some("2026-07-20".into());
    let s = snapshot(vec![goal("g1", Some("2026-08-10"))], vec![t], capacity("2026-08-01", 10, 120));
    assert_eq!(analyze(&s).overdue_task_count, 0);
}

#[test]
fn sums_todays_planned_workload() {
    let mut t1 = task("t1", "g1", Some(90), Status::NotStarted);
    t1.scheduled_date = Some("2026-08-01".into());
    let mut t2 = task("t2", "g1", Some(30), Status::NotStarted);
    t2.scheduled_date = Some("2026-08-01".into());
    let mut t3 = task("t3", "g1", Some(60), Status::NotStarted);
    t3.scheduled_date = Some("2026-08-02".into()); // tomorrow, excluded

    let s = snapshot(
        vec![goal("g1", Some("2026-08-10"))],
        vec![t1, t2, t3],
        capacity("2026-08-01", 10, 120),
    );
    let a = analyze(&s);
    assert_eq!(a.today_task_count, 2);
    assert_eq!(a.today_planned_minutes, 120);
    assert_eq!(a.today_capacity_minutes, 120);
}