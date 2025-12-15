-- Add tracking columns to detections table

-- migrate:up

ALTER TABLE detections ADD COLUMN track_id INTEGER DEFAULT -1;
ALTER TABLE detections ADD COLUMN zone_id TEXT DEFAULT '';

-- migrate:down

SELECT 1;

