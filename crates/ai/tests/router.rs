use stark_ai::action::AiAction;
use stark_ai::provider::CommandContext;
use stark_ai::router::{route, Tier};

fn ctx(instruction: &str) -> CommandContext {
    CommandContext {
        instruction: instruction.into(),
        today: "2026-08-17".into(),
        goal_names: vec!["DSA Java".into()],
        task_names: vec!["Stack Problems".into()],
    }
}

#[tokio::test]
async fn common_commands_never_reach_the_provider() {
    // No provider supplied. These must still succeed.
    for input in [
        "done stack problems",
        "log 2h graph problems",
        "move DSA to friday",
        "add task read chapter 3",
        "how am I doing",
    ] {
        let r = route(&ctx(input), None).await;
        assert!(r.is_ok(), "should have parsed locally: {input}");
        assert_eq!(r.unwrap().tier, Tier::Deterministic);
    }
}

#[tokio::test]
async fn deterministic_path_has_zero_latency() {
    let r = route(&ctx("done stack problems"), None).await.unwrap();
    assert_eq!(r.latency_ms, 0);
    assert!(r.raw.is_none());
}

#[tokio::test]
async fn unmatched_input_without_provider_is_unavailable() {
    let r = route(&ctx("reorganise my next two weeks"), None).await;
    assert!(r.is_err(), "should not have parsed locally");
}

#[tokio::test]
async fn parsed_action_is_correct() {
    let r = route(&ctx("done stack problems"), None).await.unwrap();
    match r.action {
        AiAction::CompleteTask { task_ref } => assert_eq!(task_ref, "stack problems"),
        other => panic!("wrong action: {other:?}"),
    }
}