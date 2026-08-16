use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::state::AppState;

/// Never sleep longer than this, so changes to reminders are picked up
/// within a bounded time even without an explicit wake.
const MAX_SLEEP_SECS: u64 = 60;

/// On startup, mark any pending reminder whose time has passed as MISSED.
/// They are surfaced once as a digest rather than fired as a burst of toasts.
pub fn catch_up(app: &AppHandle) {
    let state = app.state::<AppState>();
    let conn = match state.db.lock() {
        Ok(c) => c,
        Err(_) => return,
    };

    let now = stark_storage::time_util::now_utc();
    let overdue = match stark_storage::reminder_repo::overdue_pending(&conn, &now) {
        Ok(v) => v,
        Err(_) => return,
    };

    if overdue.is_empty() {
        return;
    }

    for r in &overdue {
        let _ = stark_storage::reminder_repo::set_status(
            &conn,
            &r.id,
            stark_domain::ReminderStatus::Missed,
        );
    }

    let _ = app
        .notification()
        .builder()
        .title("Stark")
        .body(&format!(
            "{} reminder(s) were missed while Stark was closed.",
            overdue.len()
        ))
        .show();
}

/// Background loop: sleep until the next reminder is due, fire it, repeat.
pub fn spawn(app: AppHandle) {
    std::thread::spawn(move || {
        loop {
            let sleep_secs = tick(&app);
            std::thread::sleep(Duration::from_secs(sleep_secs));
        }
    });
}

/// Fire anything due now; return how long to sleep before checking again.
fn tick(app: &AppHandle) -> u64 {
    let state = app.state::<AppState>();
    let now = stark_storage::time_util::now_utc();

    // Collect what needs firing, then release the lock before showing toasts.
    let due: Vec<stark_domain::Reminder> = {
        let conn = match state.db.lock() {
            Ok(c) => c,
            Err(_) => return MAX_SLEEP_SECS,
        };
        stark_storage::reminder_repo::overdue_pending(&conn, &now).unwrap_or_default()
    };

    for r in &due {
        let _ = app
            .notification()
            .builder()
            .title(&r.title)
            .body(r.body.as_deref().unwrap_or(""))
            .show();

        if let Ok(conn) = state.db.lock() {
            let _ = stark_storage::reminder_repo::set_status(
                &conn,
                &r.id,
                stark_domain::ReminderStatus::Fired,
            );
        }
    }

    if !due.is_empty() {
        let _ = app.emit("reminders-fired", due.len());
    }

    // Sleep until the next one, capped.
    let conn = match state.db.lock() {
        Ok(c) => c,
        Err(_) => return MAX_SLEEP_SECS,
    };

    match stark_storage::reminder_repo::next_pending(&conn, &now) {
        Ok(Some(next)) => {
            let secs = seconds_until(&now, &next.fire_at_utc);
            secs.clamp(1, MAX_SLEEP_SECS as i64) as u64
        }
        _ => MAX_SLEEP_SECS,
    }
}

/// Difference in seconds between two ISO-8601 UTC strings.
/// Returns 0 if `then` is in the past or either string is malformed.
fn seconds_until(now: &str, then: &str) -> i64 {
    match (parse_utc(now), parse_utc(then)) {
        (Some(a), Some(b)) => (b - a).max(0),
        _ => 0,
    }
}

/// Parse `YYYY-MM-DDTHH:MM:SSZ` into seconds since the Unix epoch.
fn parse_utc(s: &str) -> Option<i64> {
    if s.len() < 19 {
        return None;
    }
    let y: i64 = s[0..4].parse().ok()?;
    let mo: i64 = s[5..7].parse().ok()?;
    let d: i64 = s[8..10].parse().ok()?;
    let h: i64 = s[11..13].parse().ok()?;
    let mi: i64 = s[14..16].parse().ok()?;
    let sec: i64 = s[17..19].parse().ok()?;

    let days = days_from_civil(y, mo, d);
    Some(days * 86400 + h * 3600 + mi * 60 + sec)
}

fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}