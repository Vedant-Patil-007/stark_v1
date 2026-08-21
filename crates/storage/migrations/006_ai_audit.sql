-- Every mutation captured as before/after JSON. Powers undo and audit.
CREATE TABLE change_log (
    id            TEXT PRIMARY KEY,
    command_name  TEXT NOT NULL,
    actor         TEXT NOT NULL
        CHECK (actor IN ('USER','AI','ENGINE','SCHEDULER')),
    before_json   TEXT,
    after_json    TEXT,
    affected_ids  TEXT,
    is_undoable   INTEGER NOT NULL DEFAULT 1 CHECK (is_undoable IN (0,1)),
    undone_at     TEXT,
    created_at    TEXT NOT NULL
);

-- One row per AI interpretation attempt, successful or not.
CREATE TABLE ai_action (
    id                TEXT PRIMARY KEY,
    user_instruction  TEXT NOT NULL,
    provider          TEXT NOT NULL,
    model             TEXT,
    raw_response      TEXT,
    interpreted_json  TEXT,
    validation_result TEXT NOT NULL
        CHECK (validation_result IN ('ACCEPTED','REJECTED','CLARIFY')),
    validation_errors TEXT,
    change_log_id     TEXT REFERENCES change_log(id) ON DELETE SET NULL,
    outcome           TEXT NOT NULL
        CHECK (outcome IN ('EXECUTED','PENDING','CANCELLED','FAILED')),
    latency_ms        INTEGER,
    token_usage_json  TEXT,
    created_at        TEXT NOT NULL
);

CREATE INDEX idx_change_log_created ON change_log(created_at DESC);
CREATE INDEX idx_change_log_undoable ON change_log(is_undoable, undone_at);
CREATE INDEX idx_ai_action_created  ON ai_action(created_at DESC);