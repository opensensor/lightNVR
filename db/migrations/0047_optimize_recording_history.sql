-- Keep recording-history pagination and page enrichment on compact indexes.
-- The legacy indexes are repeated intentionally: migration 0025 changed after
-- its initial release, so an upgraded database must not rely on it being rerun.

-- migrate:up

CREATE INDEX IF NOT EXISTS idx_recordings_start_time
    ON recordings(start_time);
CREATE INDEX IF NOT EXISTS idx_recordings_complete_stream_start
    ON recordings(is_complete, stream_name, start_time);
CREATE INDEX IF NOT EXISTS idx_detections_stream_timestamp
    ON detections(stream_name, timestamp);
CREATE INDEX IF NOT EXISTS idx_detections_recording_id
    ON detections(recording_id);

CREATE INDEX IF NOT EXISTS idx_recordings_history_start
    ON recordings(start_time DESC)
    WHERE is_complete = 1 AND end_time IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_detections_unlinked_stream_time_label
    ON detections(stream_name, timestamp, label)
    WHERE recording_id IS NULL AND source != 'external_motion';

-- migrate:down

DROP INDEX IF EXISTS idx_detections_unlinked_stream_time_label;
DROP INDEX IF EXISTS idx_recordings_history_start;
