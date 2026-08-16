CREATE TABLE reminder (
    id           TEXT PRIMARY KEY,
    task_id      TEXT REFERENCES task(id) ON DELETE CASCADE,
    goal_id      TEXT REFERENCES goal(id) ON DELETE CASCADE,
    fire_at_utc  TEXT NOT NULL,          -- ISO-8601 UTC
    title        TEXT NOT NULL,
    body         TEXT,
    status       TEXT NOT NULL DEFAULT 'PENDING'
        CHECK (status IN ('PENDING','FIRED','MISSED','DISMISSED')),
    fired_at     TEXT,
    created_at   TEXT NOT NULL
);

CREATE INDEX idx_reminder_pending ON reminder(status, fire_at_utc);
CREATE INDEX idx_reminder_task    ON reminder(task_id);