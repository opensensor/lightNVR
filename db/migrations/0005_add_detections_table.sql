-- Add detections table for storing detection events

-- migrate:up

CREATE TABLE IF NOT EXISTS detections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stream_name TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    label TEXT NOT NULL,
    confidence REAL NOT NULL,
    x REAL,
    y REAL,
    width REAL,
    height REAL,
    recording_id INTEGER,
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (recording_id) REFERENCES recordings(id)
);

CREATE INDEX IF NOT EXISTS idx_detections_stream ON detections(stream_name);
CREATE INDEX IF NOT EXISTS idx_detections_timestamp ON detections(timestamp);
CREATE INDEX IF NOT EXISTS idx_detections_label ON detections(label);

-- migrate:down

DROP INDEX IF EXISTS idx_detections_label;
DROP INDEX IF EXISTS idx_detections_timestamp;
DROP INDEX IF EXISTS idx_detections_stream;
DROP TABLE IF EXISTS detections;

