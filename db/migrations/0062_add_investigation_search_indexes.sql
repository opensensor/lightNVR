-- Index the filtered, stable detection-search shapes used by investigations.

-- migrate:up

CREATE INDEX idx_detections_camera_label_time_id
ON detections(camera_uuid, label, timestamp DESC, id DESC)
WHERE camera_uuid IS NOT NULL;

CREATE INDEX idx_detections_camera_zone_time_id
ON detections(camera_uuid, zone_id, timestamp DESC, id DESC)
WHERE camera_uuid IS NOT NULL AND zone_id != '';

CREATE INDEX idx_detections_camera_source_time_id
ON detections(camera_uuid, source, timestamp DESC, id DESC)
WHERE camera_uuid IS NOT NULL;

-- migrate:down

DROP INDEX IF EXISTS idx_detections_camera_source_time_id;
DROP INDEX IF EXISTS idx_detections_camera_zone_time_id;
DROP INDEX IF EXISTS idx_detections_camera_label_time_id;
