-- Make restart cleanup proportional to the number of open motion intervals,
-- not to the lifetime size of the detections table.

-- migrate:up

CREATE INDEX IF NOT EXISTS idx_detections_open_external_motion
ON detections(id)
WHERE source = 'external_motion' AND event_end_time IS NULL;

-- migrate:down

DROP INDEX IF EXISTS idx_detections_open_external_motion;
