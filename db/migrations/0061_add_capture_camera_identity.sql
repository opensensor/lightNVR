-- Persist immutable camera identity on historical media and detection rows.

-- migrate:up

ALTER TABLE recordings ADD COLUMN camera_uuid TEXT;
ALTER TABLE detections ADD COLUMN camera_uuid TEXT;

-- Existing rows are safe to backfill only when their legacy stream name still
-- maps to one current camera. streams.name is unique, so unmatched rows remain
-- NULL and retain stream_name as an explicit legacy identity.
UPDATE recordings
SET camera_uuid = (
    SELECT streams.camera_uuid
    FROM streams
    WHERE streams.name = recordings.stream_name
)
WHERE camera_uuid IS NULL
  AND EXISTS (
      SELECT 1 FROM streams WHERE streams.name = recordings.stream_name
  );

UPDATE detections
SET camera_uuid = COALESCE(
    (
        SELECT recordings.camera_uuid
        FROM recordings
        WHERE recordings.id = detections.recording_id
    ),
    (
        SELECT streams.camera_uuid
        FROM streams
        WHERE streams.name = detections.stream_name
    )
)
WHERE camera_uuid IS NULL;

CREATE INDEX idx_recordings_camera_time
ON recordings(camera_uuid, start_time, end_time)
WHERE camera_uuid IS NOT NULL;

CREATE INDEX idx_detections_camera_time_id
ON detections(camera_uuid, timestamp, id)
WHERE camera_uuid IS NOT NULL;

-- migrate:down

DROP INDEX IF EXISTS idx_detections_camera_time_id;
DROP INDEX IF EXISTS idx_recordings_camera_time;
SELECT 1;
