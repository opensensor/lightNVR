-- Add indexes to improve detection filter query performance
-- The has_detection filter uses a correlated EXISTS subquery that benefits from
-- an index on recording_id (direct FK lookup) and trigger_type

-- migrate:up

CREATE INDEX IF NOT EXISTS idx_detections_recording_id ON detections(recording_id);
CREATE INDEX IF NOT EXISTS idx_recordings_trigger_type ON recordings(trigger_type);

-- migrate:down

DROP INDEX IF EXISTS idx_recordings_trigger_type;
DROP INDEX IF EXISTS idx_detections_recording_id;

