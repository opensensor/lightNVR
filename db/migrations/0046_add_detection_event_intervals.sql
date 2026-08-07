-- Persist externally-triggered detection start/stop intervals so long events
-- can span multiple recording segments and be drawn at their real times.

-- migrate:up

ALTER TABLE detections ADD COLUMN source TEXT NOT NULL DEFAULT '';
ALTER TABLE detections ADD COLUMN event_end_time INTEGER DEFAULT NULL;
-- Rows created before this migration are point detections, not open events.
UPDATE detections SET event_end_time = timestamp WHERE source = '';
CREATE INDEX IF NOT EXISTS idx_detections_external_interval
    ON detections(stream_name, source, timestamp, event_end_time);

-- migrate:down

DROP INDEX IF EXISTS idx_detections_external_interval;
ALTER TABLE detections DROP COLUMN event_end_time;
ALTER TABLE detections DROP COLUMN source;
