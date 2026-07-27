-- Settings: simple key/value, JSON payloads.
CREATE TABLE settings (
    key        TEXT PRIMARY KEY,
    value_json TEXT NOT NULL
);

CREATE TABLE goal (
    id                       TEXT PRIMARY KEY,
    title                    TEXT NOT NULL,
    description              TEXT,
    start_date               TEXT,           -- YYYY-MM-DD, local
    target_date              TEXT,           -- YYYY-MM-DD, local
    priority                 TEXT NOT NULL
        CHECK (priority IN ('LOW','MEDIUM','HIGH','CRITICAL')),
    status                   TEXT NOT NULL
        CHECK (status IN ('NOT_STARTED','IN_PROGRESS','COMPLETED','CANCELLED')),
    estimated_effort_minutes INTEGER,
    created_at               TEXT NOT NULL,  -- ISO-8601 UTC
    updated_at               TEXT NOT NULL,
    deleted_at               TEXT
);

CREATE TABLE goal_success_criterion (
    id          TEXT PRIMARY KEY,
    goal_id     TEXT NOT NULL REFERENCES goal(id) ON DELETE CASCADE,
    text        TEXT NOT NULL,
    is_met      INTEGER NOT NULL DEFAULT 0 CHECK (is_met IN (0,1)),
    met_at      TEXT,
    order_index INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE milestone (
    id          TEXT PRIMARY KEY,
    goal_id     TEXT NOT NULL REFERENCES goal(id) ON DELETE CASCADE,
    title       TEXT NOT NULL,
    description TEXT,
    target_date TEXT,
    status      TEXT NOT NULL
        CHECK (status IN ('NOT_STARTED','IN_PROGRESS','COMPLETED','CANCELLED')),
    order_index INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL,
    deleted_at  TEXT
);

CREATE TABLE task (
    id                TEXT PRIMARY KEY,
    goal_id           TEXT REFERENCES goal(id) ON DELETE SET NULL,
    milestone_id      TEXT REFERENCES milestone(id) ON DELETE SET NULL,
    title             TEXT NOT NULL,
    description       TEXT,
    due_date          TEXT,     -- when it MUST be done by
    scheduled_date    TEXT,     -- when I INTEND to do it
    estimated_minutes INTEGER,
    priority          TEXT NOT NULL
        CHECK (priority IN ('LOW','MEDIUM','HIGH','CRITICAL')),
    status            TEXT NOT NULL
        CHECK (status IN ('NOT_STARTED','IN_PROGRESS','COMPLETED','CANCELLED')),
    created_at        TEXT NOT NULL,
    updated_at        TEXT NOT NULL,
    completed_at      TEXT,
    deleted_at        TEXT
);

CREATE TABLE task_tag (
    task_id TEXT NOT NULL REFERENCES task(id) ON DELETE CASCADE,
    tag     TEXT NOT NULL,
    PRIMARY KEY (task_id, tag)
);

CREATE INDEX idx_goal_status          ON goal(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_milestone_goal       ON milestone(goal_id, order_index);
CREATE INDEX idx_task_goal_status     ON task(goal_id, status);
CREATE INDEX idx_task_due_date        ON task(due_date)       WHERE deleted_at IS NULL;
CREATE INDEX idx_task_scheduled_date  ON task(scheduled_date) WHERE deleted_at IS NULL;
CREATE INDEX idx_criterion_goal       ON goal_success_criterion(goal_id, order_index);