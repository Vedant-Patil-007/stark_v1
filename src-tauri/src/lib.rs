mod commands;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut conn = stark_storage::db::open().expect("failed to open database");

    let report = stark_storage::migrations::run_with_backup(&mut conn)
        .expect("failed to run migrations");

    if !report.applied.is_empty() {
        println!(
            "migrated {} -> {} ({} migrations applied)",
            report.from,
            report.to,
            report.applied.len()
        );
        if let Some(path) = &report.backup {
            println!("pre-migration backup: {path}");
        }
    }

    let _ = stark_storage::backup::create_daily_if_needed(&conn);
    let _ = stark_storage::backup::prune_daily(14);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new(conn))
        .invoke_handler(tauri::generate_handler![
            commands::create_goal,
            commands::list_goals,
            commands::get_goal,
            commands::goal_criteria,
            commands::delete_goal,
            commands::create_milestone,
            commands::list_milestones,
            commands::set_milestone_status,
            commands::delete_milestone,
            commands::create_task,
            commands::list_tasks,
            commands::set_task_status,
            commands::reschedule_task,
            commands::delete_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}