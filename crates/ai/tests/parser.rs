use stark_ai::action::AiAction;
use stark_ai::parser::{parse, resolve_date};

/// 2026-08-16 is a Sunday.
const TODAY: &str = "2026-08-16";

// ---------- complete ----------

#[test]
fn parses_done() {
    match parse("done stack problems", TODAY).unwrap() {
        AiAction::CompleteTask { task_ref } => assert_eq!(task_ref, "stack problems"),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_complete_and_finished() {
    for input in ["complete DSA", "finished DSA", "completed DSA"] {
        assert!(
            matches!(parse(input, TODAY), Some(AiAction::CompleteTask { .. })),
            "failed on: {input}"
        );
    }
}

#[test]
fn complete_is_case_insensitive() {
    assert!(matches!(
        parse("DONE Stack Problems", TODAY),
        Some(AiAction::CompleteTask { .. })
    ));
}

// ---------- log ----------

#[test]
fn parses_log_with_hours() {
    match parse("log 2h graph problems", TODAY).unwrap() {
        AiAction::LogWork { activity, duration_minutes, .. } => {
            assert_eq!(activity, "graph problems");
            assert_eq!(duration_minutes, Some(120));
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_log_with_minutes() {
    match parse("log 45m reading", TODAY).unwrap() {
        AiAction::LogWork { duration_minutes, .. } => {
            assert_eq!(duration_minutes, Some(45));
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_log_with_hours_and_minutes() {
    match parse("log 1h 30m revision", TODAY).unwrap() {
        AiAction::LogWork { duration_minutes, .. } => {
            assert_eq!(duration_minutes, Some(90));
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_log_with_goal() {
    match parse("log 2h graph problems on DSA Java", TODAY).unwrap() {
        AiAction::LogWork { activity, goal_ref, .. } => {
            assert_eq!(activity, "graph problems");
            assert_eq!(goal_ref.as_deref(), Some("DSA Java"));
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn log_sets_todays_date() {
    match parse("log 1h reading", TODAY).unwrap() {
        AiAction::LogWork { log_date, .. } => {
            assert_eq!(log_date.as_deref(), Some(TODAY));
        }
        _ => panic!("wrong variant"),
    }
}

// ---------- reschedule ----------

#[test]
fn parses_move_to_weekday() {
    // Today is Sunday; the next Friday is the 21st.
    match parse("move DSA to friday", TODAY).unwrap() {
        AiAction::RescheduleTask { task_ref, scheduled_date } => {
            assert_eq!(task_ref, "DSA");
            assert_eq!(scheduled_date, "2026-08-21");
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_move_to_tomorrow() {
    match parse("move stack problems to tomorrow", TODAY).unwrap() {
        AiAction::RescheduleTask { scheduled_date, .. } => {
            assert_eq!(scheduled_date, "2026-08-17");
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_move_to_iso_date() {
    match parse("move X to 2026-09-01", TODAY).unwrap() {
        AiAction::RescheduleTask { scheduled_date, .. } => {
            assert_eq!(scheduled_date, "2026-09-01");
        }
        _ => panic!("wrong variant"),
    }
}

// ---------- add task ----------

#[test]
fn parses_add_task() {
    match parse("add task read chapter 3", TODAY).unwrap() {
        AiAction::CreateTask { title, due_date, .. } => {
            assert_eq!(title, "read chapter 3");
            assert!(due_date.is_none());
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_add_task_with_due_date() {
    match parse("add task submit report by friday", TODAY).unwrap() {
        AiAction::CreateTask { title, due_date, .. } => {
            assert_eq!(title, "submit report");
            assert_eq!(due_date.as_deref(), Some("2026-08-21"));
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_add_task_with_goal() {
    match parse("add task study trees for DSA Java", TODAY).unwrap() {
        AiAction::CreateTask { title, goal_ref, .. } => {
            assert_eq!(title, "study trees");
            assert_eq!(goal_ref.as_deref(), Some("DSA Java"));
        }
        _ => panic!("wrong variant"),
    }
}

// ---------- query ----------

#[test]
fn parses_progress_queries() {
    for input in ["how am I doing", "progress", "status", "am I on track"] {
        assert!(
            matches!(parse(input, TODAY), Some(AiAction::QueryProgress { .. })),
            "failed on: {input}"
        );
    }
}

#[test]
fn parses_progress_for_specific_goal() {
    match parse("progress on DSA Java", TODAY).unwrap() {
        AiAction::QueryProgress { goal_ref } => {
            assert_eq!(goal_ref.as_deref(), Some("DSA Java"));
        }
        _ => panic!("wrong variant"),
    }
}

// ---------- no match escalates ----------

#[test]
fn returns_none_for_unrecognised_input() {
    // These must fall through to the cloud provider, not be mis-parsed.
    for input in [
        "I'm falling behind on DSA, reorganise the next two weeks",
        "what should I work on first",
        "",
        "   ",
    ] {
        assert!(parse(input, TODAY).is_none(), "should not match: {input}");
    }
}

// ---------- date resolution ----------

#[test]
fn resolves_relative_dates() {
    assert_eq!(resolve_date("today", TODAY).unwrap(), "2026-08-16");
    assert_eq!(resolve_date("tomorrow", TODAY).unwrap(), "2026-08-17");
    assert_eq!(resolve_date("yesterday", TODAY).unwrap(), "2026-08-15");
}

#[test]
fn weekday_resolves_to_next_occurrence() {
    // Today is Sunday. "sunday" means next Sunday, not today.
    assert_eq!(resolve_date("sunday", TODAY).unwrap(), "2026-08-23");
    assert_eq!(resolve_date("monday", TODAY).unwrap(), "2026-08-17");
    assert_eq!(resolve_date("saturday", TODAY).unwrap(), "2026-08-22");
}

#[test]
fn accepts_weekday_abbreviations() {
    assert_eq!(resolve_date("fri", TODAY).unwrap(), "2026-08-21");
    assert_eq!(resolve_date("wed", TODAY).unwrap(), "2026-08-19");
}

#[test]
fn rejects_nonsense_dates() {
    assert!(resolve_date("someday", TODAY).is_none());
    assert!(resolve_date("", TODAY).is_none());
}

#[test]
fn crosses_month_boundary() {
    // 2026-08-30 is a Sunday; next Tuesday is 2026-09-01.
    assert_eq!(resolve_date("tuesday", "2026-08-30").unwrap(), "2026-09-01");
}