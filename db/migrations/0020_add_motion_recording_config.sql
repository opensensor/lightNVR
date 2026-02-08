-- Add motion recording configuration tables
-- These tables support ONVIF motion-triggered recording

-- migrate:up

-- Motion recording configuration table (per-stream settings)
CREATE TABLE IF NOT EXISTS motion_recording_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_name TEXT NOT NULL UNIQUE,
    enabled INTEGER DEFAULT 1,
    pre_buffer_seconds INTEGER DEFAULT 5,
    post_buffer_seconds INTEGER DEFAULT 5,
    max_file_duration INTEGER DEFAULT 300,
    codec TEXT DEFAULT 'h264',
    quality TEXT DEFAULT 'medium',
    retention_days INTEGER DEFAULT 7,
    max_storage_mb INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE
);

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_motion_config_stream ON motion_recording_config(stream_name);

-- Motion recordings table (separate from regular recordings)
CREATE TABLE IF NOT EXISTS motion_recordings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_name TEXT NOT NULL,
    file_path TEXT NOT NULL UNIQUE,
    start_time INTEGER NOT NULL,
    end_time INTEGER,
    size_bytes INTEGER DEFAULT 0,
    width INTEGER,
    height INTEGER,
    fps INTEGER,
    codec TEXT,
    is_complete INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE
);

-- Indexes for faster queries
CREATE INDEX IF NOT EXISTS idx_motion_recordings_stream ON motion_recordings(stream_name);
CREATE INDEX IF NOT EXISTS idx_motion_recordings_time ON motion_recordings(start_time);
CREATE INDEX IF NOT EXISTS idx_motion_recordings_complete ON motion_recordings(is_complete);

-- migrate:down

DROP INDEX IF EXISTS idx_motion_recordings_complete;
DROP INDEX IF EXISTS idx_motion_recordings_time;
DROP INDEX IF EXISTS idx_motion_recordings_stream;
DROP TABLE IF EXISTS motion_recordings;
DROP INDEX IF EXISTS idx_motion_config_stream;
DROP TABLE IF EXISTS motion_recording_config;

