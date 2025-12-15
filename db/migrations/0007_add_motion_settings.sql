-- Add motion detection settings table

-- migrate:up

CREATE TABLE IF NOT EXISTS motion_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_name TEXT NOT NULL UNIQUE,
    enabled INTEGER DEFAULT 0,
    sensitivity REAL DEFAULT 0.5,
    threshold REAL DEFAULT 0.01,
    cooldown_seconds INTEGER DEFAULT 5,
    min_motion_frames INTEGER DEFAULT 3,
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- migrate:down

DROP TABLE IF EXISTS motion_settings;

