-- Fix detection zones table: migration 0008 originally created a 'zones' table
-- but the code expects 'detection_zones' with a different schema.
-- This migration handles existing databases that applied the old 0008.

-- migrate:up

DROP TABLE IF EXISTS zones;

CREATE TABLE IF NOT EXISTS detection_zones (
    id TEXT PRIMARY KEY,
    stream_name TEXT NOT NULL,
    name TEXT NOT NULL,
    enabled INTEGER DEFAULT 1,
    color TEXT DEFAULT '#3b82f6',
    polygon TEXT NOT NULL,
    filter_classes TEXT DEFAULT '',
    min_confidence REAL DEFAULT 0.0,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_detection_zones_stream ON detection_zones(stream_name);

-- migrate:down

DROP INDEX IF EXISTS idx_detection_zones_stream;
DROP TABLE IF EXISTS detection_zones;

