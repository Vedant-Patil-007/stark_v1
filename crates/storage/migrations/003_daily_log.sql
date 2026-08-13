CREATE TABLE daily_log_entry (
    id               TEXT PRIMARY KEY,
    log_date         TEXT NOT NULL,          -- YYYY-MM-DD, local
    task_id          TEXT REFERENCES task(id) ON DELETE SET NULL,
    milestone_id     TEXT REFERENCES milestone(id) ON DELETE SET NULL,
    goal_id          TEXT REFERENCES goal(id) ON DELETE SET NULL,
    activity         TEXT NOT NULL,
    duration_minutes INTEGER,
    category         TEXT,
    notes            TEXT,
    created_at       TEXT NOT NULL,
    updated_at       TEXT NOT NULL
);

CREATE INDEX idx_log_date      ON daily_log_entry(log_date);
CREATE INDEX idx_log_goal      ON daily_log_entry(goal_id, log_date);
CREATE INDEX idx_log_task      ON daily_log_entry(task_id);