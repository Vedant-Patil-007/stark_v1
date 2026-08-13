-- Recurring weekly working hours. weekday: 0=Sunday .. 6=Saturday.
-- Times are minutes from local midnight (0..1440).
CREATE TABLE availability_template (
    id           TEXT PRIMARY KEY,
    weekday      INTEGER NOT NULL CHECK (weekday BETWEEN 0 AND 6),
    start_minute INTEGER NOT NULL CHECK (start_minute BETWEEN 0 AND 1440),
    end_minute   INTEGER NOT NULL CHECK (end_minute BETWEEN 0 AND 1440),
    label        TEXT,
    created_at   TEXT NOT NULL,
    CHECK (end_minute > start_minute)
);

-- Date-specific overrides. is_available = 0 carves time OUT of the template
-- (e.g. "unavailable Tuesday 2pm-6pm"); is_available = 1 adds extra time.
CREATE TABLE availability_exception (
    id           TEXT PRIMARY KEY,
    date         TEXT NOT NULL,              -- YYYY-MM-DD, local
    start_minute INTEGER NOT NULL CHECK (start_minute BETWEEN 0 AND 1440),
    end_minute   INTEGER NOT NULL CHECK (end_minute BETWEEN 0 AND 1440),
    is_available INTEGER NOT NULL DEFAULT 0 CHECK (is_available IN (0,1)),
    note         TEXT,
    created_at   TEXT NOT NULL,
    CHECK (end_minute > start_minute)
);

CREATE INDEX idx_availability_weekday   ON availability_template(weekday);
CREATE INDEX idx_availability_exception ON availability_exception(date);