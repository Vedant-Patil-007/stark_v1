use rusqlite::Connection;
use std::sync::Mutex;

/// The single database connection, shared across all commands.
/// SQLite handles one writer at a time; the Mutex makes that explicit.
pub struct AppState {
    pub db: Mutex<Connection>,
}

impl AppState {
    pub fn new(conn: Connection) -> Self {
        Self {
            db: Mutex::new(conn),
        }
    }
}