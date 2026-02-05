-- Add recording_id column to detections table for linking detections to recordings
-- This supports annotation mode where detections are linked to continuous recordings

-- migrate:up

ALTER TABLE detections ADD COLUMN recording_id INTEGER REFERENCES recordings(id);

-- migrate:down

SELECT 1;

