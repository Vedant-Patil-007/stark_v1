use stark_commands::{goal as goal_cmd, ErrorPayload};
use stark_domain::{Goal, GoalId, NewGoal, SuccessCriterion};
use tauri::State;
use stark_commands::task as task_cmd;
use stark_domain::{NewTask, Task, TaskFilter, TaskId};
use crate::state::AppState;
use stark_commands::daily_log as log_cmd;
use stark_domain::{LogEntry, LogEntryId, NewLogEntry};
use stark_commands::milestone as milestone_cmd;
use stark_domain::{Milestone, MilestoneId, NewMilestone, Status};
use stark_commands::availability as avail_cmd;
use stark_domain::{
    AvailabilityException, AvailabilityId, AvailabilityWindow, DayCapacity, ExceptionId,
    NewAvailabilityException, NewAvailabilityWindow,
};
use stark_commands::planning as planning_cmd;
use stark_planning::Analysis;
type CmdResult<T> = std::result::Result<T, ErrorPayload>;

#[tauri::command]
pub fn create_goal(state: State<'_, AppState>, input: NewGoal) -> CmdResult<Goal> {
    let mut conn = state.db.lock().unwrap();
    goal_cmd::create_goal(&mut conn, input).map_err(Into::into)
}

#[tauri::command]
pub fn list_goals(state: State<'_, AppState>) -> CmdResult<Vec<Goal>> {
    let conn = state.db.lock().unwrap();
    goal_cmd::list_goals(&conn).map_err(Into::into)
}

#[tauri::command]
pub fn get_goal(state: State<'_, AppState>, id: String) -> CmdResult<Goal> {
    let conn = state.db.lock().unwrap();
    goal_cmd::get_goal(&conn, &GoalId::from(id)).map_err(Into::into)
}

#[tauri::command]
pub fn goal_criteria(state: State<'_, AppState>, id: String) -> CmdResult<Vec<SuccessCriterion>> {
    let conn = state.db.lock().unwrap();
    goal_cmd::goal_criteria(&conn, &GoalId::from(id)).map_err(Into::into)
}

#[tauri::command]
pub fn delete_goal(state: State<'_, AppState>, id: String) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    goal_cmd::delete_goal(&conn, &GoalId::from(id)).map_err(Into::into)
}



#[tauri::command]
pub fn create_milestone(
    state: State<'_, AppState>,
    input: NewMilestone,
) -> CmdResult<Milestone> {
    let conn = state.db.lock().unwrap();
    milestone_cmd::create_milestone(&conn, input).map_err(Into::into)
}

#[tauri::command]
pub fn list_milestones(
    state: State<'_, AppState>,
    goal_id: String,
) -> CmdResult<Vec<Milestone>> {
    let conn = state.db.lock().unwrap();
    milestone_cmd::list_milestones(&conn, &GoalId::from(goal_id)).map_err(Into::into)
}

#[tauri::command]
pub fn set_milestone_status(
    state: State<'_, AppState>,
    id: String,
    status: Status,
) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    milestone_cmd::set_milestone_status(&conn, &MilestoneId::from(id), status)
        .map_err(Into::into)
}

#[tauri::command]
pub fn delete_milestone(state: State<'_, AppState>, id: String) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    milestone_cmd::delete_milestone(&conn, &MilestoneId::from(id)).map_err(Into::into)
}
#[tauri::command]
pub fn create_task(state: State<'_, AppState>, input: NewTask) -> CmdResult<Task> {
    let conn = state.db.lock().unwrap();
    task_cmd::create_task(&conn, input).map_err(Into::into)
}

#[tauri::command]
pub fn list_tasks(state: State<'_, AppState>, filter: TaskFilter) -> CmdResult<Vec<Task>> {
    let conn = state.db.lock().unwrap();
    task_cmd::list_tasks(&conn, &filter).map_err(Into::into)
}

#[tauri::command]
pub fn set_task_status(
    state: State<'_, AppState>,
    id: String,
    status: Status,
) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    task_cmd::set_task_status(&conn, &TaskId::from(id), status).map_err(Into::into)
}

#[tauri::command]
pub fn reschedule_task(
    state: State<'_, AppState>,
    id: String,
    scheduled_date: Option<String>,
) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    task_cmd::reschedule_task(&conn, &TaskId::from(id), scheduled_date).map_err(Into::into)
}

#[tauri::command]
pub fn delete_task(state: State<'_, AppState>, id: String) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    task_cmd::delete_task(&conn, &TaskId::from(id)).map_err(Into::into)
}


#[tauri::command]
pub fn create_log_entry(
    state: State<'_, AppState>,
    input: NewLogEntry,
) -> CmdResult<LogEntry> {
    let conn = state.db.lock().unwrap();
    log_cmd::create_log_entry(&conn, input).map_err(Into::into)
}

#[tauri::command]
pub fn list_log_for_date(
    state: State<'_, AppState>,
    date: String,
) -> CmdResult<Vec<LogEntry>> {
    let conn = state.db.lock().unwrap();
    log_cmd::list_log_for_date(&conn, &date).map_err(Into::into)
}

#[tauri::command]
pub fn delete_log_entry(state: State<'_, AppState>, id: String) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    log_cmd::delete_log_entry(&conn, &LogEntryId::from(id)).map_err(Into::into)
}
#[tauri::command]
pub fn tasks_in_range(
    state: State<'_, AppState>,
    from: String,
    to: String,
) -> CmdResult<Vec<Task>> {
    let conn = state.db.lock().unwrap();
    task_cmd::tasks_in_range(&conn, &from, &to).map_err(Into::into)
}



#[tauri::command]
pub fn create_availability_window(
    state: State<'_, AppState>,
    input: NewAvailabilityWindow,
) -> CmdResult<AvailabilityWindow> {
    let conn = state.db.lock().unwrap();
    avail_cmd::create_availability_window(&conn, input).map_err(Into::into)
}

#[tauri::command]
pub fn list_availability_windows(
    state: State<'_, AppState>,
) -> CmdResult<Vec<AvailabilityWindow>> {
    let conn = state.db.lock().unwrap();
    avail_cmd::list_availability_windows(&conn).map_err(Into::into)
}

#[tauri::command]
pub fn delete_availability_window(
    state: State<'_, AppState>,
    id: String,
) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    avail_cmd::delete_availability_window(&conn, &AvailabilityId::from(id))
        .map_err(Into::into)
}

#[tauri::command]
pub fn create_availability_exception(
    state: State<'_, AppState>,
    input: NewAvailabilityException,
) -> CmdResult<AvailabilityException> {
    let conn = state.db.lock().unwrap();
    avail_cmd::create_availability_exception(&conn, input).map_err(Into::into)
}

#[tauri::command]
pub fn list_availability_exceptions(
    state: State<'_, AppState>,
    from: String,
    to: String,
) -> CmdResult<Vec<AvailabilityException>> {
    let conn = state.db.lock().unwrap();
    avail_cmd::list_availability_exceptions(&conn, &from, &to).map_err(Into::into)
}

#[tauri::command]
pub fn delete_availability_exception(
    state: State<'_, AppState>,
    id: String,
) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    avail_cmd::delete_availability_exception(&conn, &ExceptionId::from(id))
        .map_err(Into::into)
}

#[tauri::command]
pub fn capacity_for_date(
    state: State<'_, AppState>,
    date: String,
    weekday: i64,
) -> CmdResult<DayCapacity> {
    let conn = state.db.lock().unwrap();
    avail_cmd::capacity_for_date(&conn, &date, weekday).map_err(Into::into)
}



#[tauri::command]
pub fn analyze_plan(state: State<'_, AppState>, today: String) -> CmdResult<Analysis> {
    let conn = state.db.lock().unwrap();
    planning_cmd::analyze_plan(&conn, &today).map_err(Into::into)
}
#[tauri::command]
pub fn today_tasks(state: State<'_, AppState>, today: String) -> CmdResult<Vec<Task>> {
    let conn = state.db.lock().unwrap();
    planning_cmd::today_tasks(&conn, &today).map_err(Into::into)
}

#[tauri::command]
pub fn overdue_tasks(state: State<'_, AppState>, today: String) -> CmdResult<Vec<Task>> {
    let conn = state.db.lock().unwrap();
    planning_cmd::overdue_tasks(&conn, &today).map_err(Into::into)
}

use stark_domain::{Reminder, ReminderId};

#[tauri::command]
pub fn sync_reminders(
    state: State<'_, AppState>,
    date: String,
    offset_minutes: i64,
) -> CmdResult<usize> {
    let conn = state.db.lock().unwrap();
    planning_cmd::sync_reminders_for_date(&conn, &date, offset_minutes).map_err(Into::into)
}

#[tauri::command]
pub fn list_missed_reminders(state: State<'_, AppState>) -> CmdResult<Vec<Reminder>> {
    let conn = state.db.lock().unwrap();
    planning_cmd::list_missed_reminders(&conn).map_err(Into::into)
}

#[tauri::command]
pub fn dismiss_reminder(state: State<'_, AppState>, id: String) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    planning_cmd::dismiss_reminder(&conn, &ReminderId::from(id)).map_err(Into::into)
}


use stark_ai::keys;
use stark_ai::nvidia::NvidiaProvider;
use stark_ai::provider::CommandContext;
use stark_ai::router::{route, Tier};
use stark_commands::ai_bridge::{self, ApplyOutcome};

const DEFAULT_MODEL: &str = "nvidia/nemotron-3-nano-30b-a3b";

#[derive(serde::Serialize)]
pub struct AiResult {
    pub outcome: ApplyOutcome,
    pub tier: String,
    pub latency_ms: u64,
    pub action_name: String,
}

#[tauri::command]
pub async fn run_ai_command(
    state: State<'_, AppState>,
    instruction: String,
    today: String,
) -> CmdResult<AiResult> {
    // Build context from the current database, then release the lock
    // before any network call.
    let ctx = {
        let conn = state.db.lock().unwrap();
        let goals = stark_storage::goal_repo::list(&conn)
            .map_err(|e| ErrorPayload { kind: "STORAGE".into(), message: e.to_string() })?;
        let tasks = stark_storage::task_repo::list(
            &conn,
            &stark_domain::TaskFilter {
                goal_id: None,
                milestone_id: None,
                scheduled_date: None,
                include_completed: false,
            },
        )
        .map_err(|e| ErrorPayload { kind: "STORAGE".into(), message: e.to_string() })?;

        CommandContext {
            instruction: instruction.clone(),
            today: today.clone(),
            goal_names: goals.into_iter().map(|g| g.title).collect(),
            task_names: tasks.into_iter().map(|t| t.title).collect(),
        }
    };

       let provider = {
        let conn = state.db.lock().unwrap();
        keys::load_key(&conn, "nvidia")
            .ok()
            .flatten()
            .map(|k| NvidiaProvider::new(k, DEFAULT_MODEL.to_string()))
    };

    let routed = route(&ctx, provider.as_ref().map(|p| p as &dyn stark_ai::provider::AiProvider))
        .await
        .map_err(|e| ErrorPayload {
            kind: "AI".into(),
            message: e.to_string(),
        })?;

    let action_name = routed.action.name().to_string();
    let tier = match routed.tier {
        Tier::Deterministic => "local",
        Tier::Remote => "cloud",
    };

    let outcome = {
        let mut conn = state.db.lock().unwrap();
               ai_bridge::apply(&mut conn, routed.action, &today)
            .map_err(ErrorPayload::from)?
    };

    Ok(AiResult {
        outcome,
        tier: tier.to_string(),
        latency_ms: routed.latency_ms,
        action_name,
    })
}

#[tauri::command]
pub fn set_ai_key(state: State<'_, AppState>, key: String) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    keys::store_key(&conn, "nvidia", &key).map_err(|e| ErrorPayload {
        kind: "AI".into(),
        message: e.to_string(),
    })
}

#[tauri::command]
pub fn has_ai_key(state: State<'_, AppState>) -> String {
    let conn = state.db.lock().unwrap();
    match keys::load_key(&conn, "nvidia") {
        Ok(Some(k)) => format!("found, {} chars", k.len()),
        Ok(None) => "no entry".into(),
        Err(e) => format!("error: {e}"),
    }
}

#[tauri::command]
pub fn clear_ai_key(state: State<'_, AppState>) -> CmdResult<()> {
    let conn = state.db.lock().unwrap();
    keys::delete_key(&conn, "nvidia").map_err(|e| ErrorPayload {
        kind: "AI".into(),
        message: e.to_string(),
    })
}