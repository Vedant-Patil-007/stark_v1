mod commands;
mod scheduler;
mod state;

use state::AppState;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

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
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Someone launched a second copy: focus the existing window instead.
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(AppState::new(conn))
        .setup(|app| {
            let handle = app.handle().clone();

// ---- TEMPORARY TEST: delete once the toast has been seen ----
            {
                let state = handle.state::<AppState>();
                let conn = state.db.lock().unwrap();

                // Start from a clean slate so old reminders don't confuse the test.
                let _ = conn.execute("DELETE FROM reminder", []);

                let now = stark_storage::time_util::now_utc();

                // Push the fire time 30 seconds into the future.
                let fire_at = {
                    let mut s = now.clone();
                    let secs: i64 = s[17..19].parse().unwrap_or(0);
                    let mins: i64 = s[14..16].parse().unwrap_or(0);
                    let total = secs + 30;
                    let new_secs = total % 60;
                    let new_mins = (mins + total / 60) % 60;
                    s.replace_range(17..19, &format!("{:02}", new_secs));
                    s.replace_range(14..16, &format!("{:02}", new_mins));
                    s
                };

                println!("test reminder scheduled for {fire_at} (now is {now})");

                let _ = stark_storage::reminder_repo::create(
                    &conn,
                    stark_domain::NewReminder {
                        task_id: None,
                        goal_id: None,
                        fire_at_utc: fire_at,
                        title: "Stark test reminder".into(),
                        body: Some("If you see this, the scheduler works.".into()),
                    },
                );
            }
            // ---- END TEMPORARY TEST ----

            // ---- system tray ----
            let show = MenuItem::with_id(app, "show", "Open Stark", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("Stark")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            // ---- reminders ----
            scheduler::catch_up(&handle);
            scheduler::spawn(handle);

            Ok(())
        })
        .on_window_event(|window, event| {
            // Closing the window hides it; the app keeps running in the tray
            // so reminders continue to fire. Quit from the tray menu.
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
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
            commands::create_log_entry,
            commands::list_log_for_date,
            commands::delete_log_entry,
            commands::tasks_in_range,
            commands::create_availability_window,
            commands::list_availability_windows,
            commands::delete_availability_window,
            commands::create_availability_exception,
            commands::list_availability_exceptions,
            commands::delete_availability_exception,
            commands::capacity_for_date,
            commands::analyze_plan,
            commands::today_tasks,
            commands::overdue_tasks,
            commands::sync_reminders,
            commands::list_missed_reminders,
            commands::dismiss_reminder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}