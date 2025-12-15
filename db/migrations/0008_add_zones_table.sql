-- Add detection zones table

-- migrate:up

CREATE TABLE IF NOT EXISTS zones (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_name TEXT NOT NULL,
    name TEXT NOT NULL,
    enabled INTEGER DEFAULT 1,
    type TEXT DEFAULT 'detection',
    points TEXT NOT NULL,
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_zones_stream ON zones(stream_name);

-- migrate:down

DROP INDEX IF EXISTS idx_zones_stream;
DROP TABLE IF EXISTS zones;

