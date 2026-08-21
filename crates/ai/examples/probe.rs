//! Reliability probe. Fires representative commands at the provider and
//! reports how many produce valid, correctly-typed actions.
//!
//! Run with:  cargo run -p stark-ai --example probe

use stark_ai::nvidia::NvidiaProvider;
use stark_ai::provider::{AiProvider, CommandContext};

#[tokio::main]
async fn main() {
    let key = match std::env::var("NVIDIA_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            eprintln!("Set NVIDIA_API_KEY first:");
            eprintln!("  $env:NVIDIA_API_KEY = \"nvapi-...\"");
            std::process::exit(1);
        }
    };

    let provider = NvidiaProvider::new(key, "nvidia/nemotron-3-nano-30b-a3b".into());

    let goals = vec![
        "DSA Java".to_string(),
        "Research Project".to_string(),
        "University Setup".to_string(),
    ];
    let tasks = vec![
        "Stack Problems".to_string(),
        "Literature Review".to_string(),
        "Study Trees".to_string(),
    ];

    // (input, what a correct answer looks like)
    let cases = [
        ("Tomorrow I need to study graphs for 2 hours as part of my DSA goal", "CREATE_TASK with goal_ref DSA Java, 120 min"),
        ("I finished the stack problems", "COMPLETE_TASK"),
        ("Move Study Trees to Friday", "RESCHEDULE_TASK"),
        ("Today I solved 7 array problems and spent an hour on it", "LOG_WORK ~60 min"),
        ("I'm unavailable tomorrow from 2 to 6", "SET_AVAILABILITY 840-1080, is_available false"),
        ("How am I doing on the research project", "QUERY_PROGRESS"),
        ("Create a goal to finish Java by October", "CREATE_GOAL"),
        ("Look into internship applications", "CREATE_INBOX_ITEM (too vague)"),
        ("Mark the DSA thing as done", "NEEDS_CLARIFICATION or COMPLETE_TASK"),
        ("What's the weather like", "CREATE_INBOX_ITEM or NEEDS_CLARIFICATION"),
    ];

    let mut ok = 0;
    let mut total_ms = 0u64;

    for (input, expected) in cases {
        let ctx = CommandContext {
            instruction: input.to_string(),
            today: "2026-08-17".into(),
            goal_names: goals.clone(),
            task_names: tasks.clone(),
        };

        print!("\n─── {input}\n    expect: {expected}\n    ");

        match provider.interpret(&ctx).await {
            Ok(r) => {
                ok += 1;
                total_ms += r.latency_ms;
                println!("OK  ({} ms)", r.latency_ms);
                println!("    {:?}", r.action);
            }
            Err(e) => {
                println!("FAIL");
                println!("    {e}");
            }
        }
    }

    println!("\n═══════════════════════════════");
    println!("parsed: {ok}/10");
    if ok > 0 {
        println!("avg latency: {} ms", total_ms / ok as u64);
    }
}