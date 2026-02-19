-- Consolidate all indexes into migrations
-- Adds missing indexes that were previously only created in C code (db_core.c, db_detections.c)
-- Also adds new performance indexes for detection filter queries

-- migrate:up

-- Detection performance indexes (new)
CREATE INDEX IF NOT EXISTS idx_detections_recording_id ON detections(recording_id);

-- Indexes previously only in db_core.c (never migrated)
CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp);
CREATE INDEX IF NOT EXISTS idx_events_type ON events(type);
CREATE INDEX IF NOT EXISTS idx_events_stream ON events(stream_name);
CREATE INDEX IF NOT EXISTS idx_recordings_end_time ON recordings(end_time);
CREATE INDEX IF NOT EXISTS idx_recordings_complete_stream_start ON recordings(is_complete, stream_name, start_time);
CREATE INDEX IF NOT EXISTS idx_streams_name ON streams(name);
CREATE INDEX IF NOT EXISTS idx_detections_stream_timestamp ON detections(stream_name, timestamp);

-- migrate:down

DROP INDEX IF EXISTS idx_detections_stream_timestamp;
DROP INDEX IF EXISTS idx_streams_name;
DROP INDEX IF EXISTS idx_recordings_complete_stream_start;
DROP INDEX IF EXISTS idx_recordings_end_time;
DROP INDEX IF EXISTS idx_events_stream;
DROP INDEX IF EXISTS idx_events_type;
DROP INDEX IF EXISTS idx_events_timestamp;
DROP INDEX IF EXISTS idx_detections_recording_id;

