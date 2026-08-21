use crate::action::AiAction;
use crate::error::{AiError, Result};

/// What the model needs to know to interpret a command.
/// Deliberately minimal: only names relevant to the request go to the cloud,
/// never the full database.
#[derive(Debug, Clone)]
pub struct CommandContext {
    pub instruction: String,
    /// Local date, YYYY-MM-DD.
    pub today: String,
    /// Goal titles the user currently has.
    pub goal_names: Vec<String>,
    /// Titles of open tasks.
    pub task_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProviderResponse {
    pub action: AiAction,
    pub raw: String,
    pub model: String,
    pub latency_ms: u64,
}

#[async_trait::async_trait]
pub trait AiProvider: Send + Sync {
    async fn interpret(&self, ctx: &CommandContext) -> Result<ProviderResponse>;
    fn name(&self) -> &'static str;
}

/// Build the system prompt. Kept in one place so it can be iterated on
/// without touching provider code.
pub fn system_prompt(ctx: &CommandContext) -> String {
    let goals = if ctx.goal_names.is_empty() {
        "(none)".to_string()
    } else {
        ctx.goal_names.join(", ")
    };
    let tasks = if ctx.task_names.is_empty() {
        "(none)".to_string()
    } else {
        ctx.task_names.join(", ")
    };

    format!(
        r#"You convert a user's request into exactly one JSON action for a personal planner.

Today is {today}.

The user's goals: {goals}
The user's open tasks: {tasks}

Respond with ONE JSON object and nothing else. No prose, no markdown fences.

Allowed actions:

{{"action":"CREATE_TASK","title":"...","goal_ref":null,"due_date":null,"scheduled_date":null,"estimated_minutes":null,"priority":null}}
{{"action":"COMPLETE_TASK","task_ref":"..."}}
{{"action":"RESCHEDULE_TASK","task_ref":"...","scheduled_date":"YYYY-MM-DD"}}
{{"action":"CREATE_GOAL","title":"...","target_date":null,"priority":null}}
{{"action":"LOG_WORK","activity":"...","duration_minutes":null,"goal_ref":null,"log_date":null}}
{{"action":"SET_AVAILABILITY","date":"YYYY-MM-DD","start_minute":0,"end_minute":0,"is_available":false}}
{{"action":"QUERY_PROGRESS","goal_ref":null}}
{{"action":"CREATE_INBOX_ITEM","raw_text":"..."}}
{{"action":"NEEDS_CLARIFICATION","question":"...","candidates":[]}}

Rules:
- goal_ref and task_ref are NAMES copied from the lists above. Never invent an ID.
- Dates are YYYY-MM-DD. Resolve relative dates against today.
- priority is one of LOW, MEDIUM, HIGH, CRITICAL, or null.
- Times are minutes from midnight (9am = 540, 5pm = 1020).
- If the request is too vague to act on, use CREATE_INBOX_ITEM.
- If it is clear but the reference is ambiguous, use NEEDS_CLARIFICATION.
- Never guess a duration, deadline, or goal the user did not state.
- Include only the fields shown. Do not add extra fields."#,
        today = ctx.today,
        goals = goals,
        tasks = tasks,
    )
}

/// Models often wrap JSON in markdown fences or add a sentence.
/// Extract the first balanced JSON object.
pub fn extract_json(raw: &str) -> Result<&str> {
    let start = raw
        .find('{')
        .ok_or_else(|| AiError::Parse("no JSON object in response".into()))?;

    let bytes = raw.as_bytes();
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escaped = false;

    for i in start..bytes.len() {
        let ch = bytes[i] as char;
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '{' if !in_string => depth += 1,
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return Ok(&raw[start..=i]);
                }
            }
            _ => {}
        }
    }

    Err(AiError::Parse("unbalanced JSON in response".into()))
}