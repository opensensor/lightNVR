-- Add detection-based recording columns to streams table

-- migrate:up

ALTER TABLE streams ADD COLUMN detection_based_recording INTEGER DEFAULT 0;
ALTER TABLE streams ADD COLUMN detection_model TEXT DEFAULT '';
ALTER TABLE streams ADD COLUMN detection_threshold REAL DEFAULT 0.5;
ALTER TABLE streams ADD COLUMN detection_interval INTEGER DEFAULT 10;
ALTER TABLE streams ADD COLUMN pre_detection_buffer INTEGER DEFAULT 0;
ALTER TABLE streams ADD COLUMN post_detection_buffer INTEGER DEFAULT 3;

-- migrate:down

-- SQLite doesn't support DROP COLUMN before 3.35.0
-- For older versions, we'd need to recreate the table
-- This is a no-op for safety on older SQLite versions
SELECT 1;

