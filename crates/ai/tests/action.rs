use stark_ai::AiAction;

fn parse(json: &str) -> Result<AiAction, serde_json::Error> {
    serde_json::from_str(json)
}

// ---------- valid actions deserialize ----------

#[test]
fn parses_minimal_create_task() {
    let a = parse(r#"{"action":"CREATE_TASK","title":"Read chapter 3"}"#).unwrap();
    match a {
        AiAction::CreateTask { title, goal_ref, due_date, .. } => {
            assert_eq!(title, "Read chapter 3");
            assert!(goal_ref.is_none());
            assert!(due_date.is_none());
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_full_create_task() {
    let json = r#"{
        "action":"CREATE_TASK",
        "title":"Study graphs",
        "goal_ref":"DSA Java",
        "due_date":"2026-09-01",
        "scheduled_date":"2026-08-28",
        "estimated_minutes":120,
        "priority":"HIGH"
    }"#;
    let a = parse(json).unwrap();
    match a {
        AiAction::CreateTask { estimated_minutes, goal_ref, .. } => {
            assert_eq!(estimated_minutes, Some(120));
            assert_eq!(goal_ref.as_deref(), Some("DSA Java"));
        }
        _ => panic!("wrong variant"),
    }
}

#[test]
fn parses_log_work() {
    let json = r#"{"action":"LOG_WORK","activity":"Solved 7 array problems","duration_minutes":120}"#;
    assert!(matches!(parse(json).unwrap(), AiAction::LogWork { .. }));
}

#[test]
fn parses_needs_clarification_with_candidates() {
    let json = r#"{
        "action":"NEEDS_CLARIFICATION",
        "question":"Which goal did you mean?",
        "candidates":["DSA Java","DSA Python"]
    }"#;
    match parse(json).unwrap() {
        AiAction::NeedsClarification { candidates, .. } => {
            assert_eq!(candidates.len(), 2);
        }
        _ => panic!("wrong variant"),
    }
}

// ---------- malformed output is rejected ----------

#[test]
fn rejects_unknown_action() {
    // A model inventing a capability we never gave it.
    let json = r#"{"action":"DELETE_EVERYTHING"}"#;
    assert!(parse(json).is_err());
}

#[test]
fn rejects_unknown_field() {
    // deny_unknown_fields: extra keys are a hard failure, not silently dropped.
    let json = r#"{"action":"CREATE_TASK","title":"X","sql":"DROP TABLE task"}"#;
    assert!(parse(json).is_err());
}

#[test]
fn rejects_missing_required_field() {
    let json = r#"{"action":"COMPLETE_TASK"}"#;
    assert!(parse(json).is_err());
}

#[test]
fn rejects_wrong_type() {
    let json = r#"{"action":"CREATE_TASK","title":"X","estimated_minutes":"two hours"}"#;
    assert!(parse(json).is_err());
}

#[test]
fn rejects_raw_prose() {
    assert!(parse("Sure! I'll create that task for you.").is_err());
}

#[test]
fn rejects_empty_input() {
    assert!(parse("").is_err());
}

#[test]
fn rejects_invalid_priority() {
    let json = r#"{"action":"CREATE_TASK","title":"X","priority":"URGENT"}"#;
    assert!(parse(json).is_err());
}

// ---------- the ID rule ----------

#[test]
fn action_enum_has_no_id_fields() {
    // A model must never emit a database ID. This test documents that rule;
    // if someone adds an `id` field to the enum, it should fail review.
    let json = r#"{"action":"COMPLETE_TASK","task_id":"0192f8a1-1234-7890-abcd-ef0123456789"}"#;
    assert!(
        parse(json).is_err(),
        "task_id must not be accepted; references are names, resolved by lookup"
    );
}

// ---------- risk classification ----------

#[test]
fn create_task_is_low_impact() {
    let a = parse(r#"{"action":"CREATE_TASK","title":"X"}"#).unwrap();
    assert!(!a.is_high_impact());
}

#[test]
fn set_availability_is_high_impact() {
    let json = r#"{"action":"SET_AVAILABILITY","date":"2026-08-20","start_minute":840,"end_minute":1080,"is_available":false}"#;
    assert!(parse(json).unwrap().is_high_impact());
}