-- Initial database schema for lightNVR
-- This creates the base tables needed for the NVR system

-- migrate:up

-- Streams configuration table
CREATE TABLE IF NOT EXISTS streams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    url TEXT NOT NULL,
    enabled INTEGER DEFAULT 1,
    streaming_enabled INTEGER DEFAULT 1,
    width INTEGER DEFAULT 1280,
    height INTEGER DEFAULT 720,
    fps INTEGER DEFAULT 30,
    codec TEXT DEFAULT 'h264',
    priority INTEGER DEFAULT 5,
    record INTEGER DEFAULT 1,
    segment_duration INTEGER DEFAULT 900
);

-- Recordings table
CREATE TABLE IF NOT EXISTS recordings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER,
    size_bytes INTEGER DEFAULT 0,
    width INTEGER,
    height INTEGER,
    codec TEXT,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- Create index for efficient queries
CREATE INDEX IF NOT EXISTS idx_recordings_stream_name ON recordings(stream_name);
CREATE INDEX IF NOT EXISTS idx_recordings_start_time ON recordings(start_time);

-- migrate:down

DROP INDEX IF EXISTS idx_recordings_start_time;
DROP INDEX IF EXISTS idx_recordings_stream_name;
DROP TABLE IF EXISTS recordings;
DROP TABLE IF EXISTS streams;

