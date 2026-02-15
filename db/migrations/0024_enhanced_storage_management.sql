-- Enhanced storage management: tiered retention, disk pressure, daily stats

-- migrate:up

-- Tiered retention columns on recordings
ALTER TABLE recordings ADD COLUMN retention_tier INTEGER DEFAULT 2;
ALTER TABLE recordings ADD COLUMN disk_pressure_eligible INTEGER DEFAULT 1;

-- Tier multipliers and storage priority on streams
ALTER TABLE streams ADD COLUMN tier_critical_multiplier REAL DEFAULT 3.0;
ALTER TABLE streams ADD COLUMN tier_important_multiplier REAL DEFAULT 2.0;
ALTER TABLE streams ADD COLUMN tier_ephemeral_multiplier REAL DEFAULT 0.25;
ALTER TABLE streams ADD COLUMN storage_priority INTEGER DEFAULT 5;

-- Daily storage statistics table for analytics
CREATE TABLE IF NOT EXISTS storage_daily_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
    stream_name TEXT NOT NULL,
    tier INTEGER NOT NULL DEFAULT 2,
    recording_count INTEGER NOT NULL DEFAULT 0,
    total_bytes INTEGER NOT NULL DEFAULT 0,
    deleted_count INTEGER NOT NULL DEFAULT 0,
    deleted_bytes INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for efficient queries on daily stats
CREATE INDEX IF NOT EXISTS idx_storage_daily_stats_stream_date ON storage_daily_stats(stream_name, date);
CREATE INDEX IF NOT EXISTS idx_storage_daily_stats_date_tier ON storage_daily_stats(date, tier);

-- Index for tier-based retention queries
CREATE INDEX IF NOT EXISTS idx_recordings_retention_tier ON recordings(retention_tier);
CREATE INDEX IF NOT EXISTS idx_recordings_disk_pressure ON recordings(disk_pressure_eligible);

-- migrate:down

DROP INDEX IF EXISTS idx_recordings_disk_pressure;
DROP INDEX IF EXISTS idx_recordings_retention_tier;
DROP INDEX IF EXISTS idx_storage_daily_stats_date_tier;
DROP INDEX IF EXISTS idx_storage_daily_stats_stream_date;
DROP TABLE IF EXISTS storage_daily_stats;
SELECT 1;

