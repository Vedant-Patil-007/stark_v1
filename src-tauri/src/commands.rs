use stark_commands::{goal as goal_cmd, ErrorPayload};
use stark_domain::{Goal, GoalId, NewGoal, SuccessCriterion};
use tauri::State;

use crate::state::AppState;

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