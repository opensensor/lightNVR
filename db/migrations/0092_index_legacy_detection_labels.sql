-- Keep the capture-identity label fallback bounded by distinct legacy labels,
-- without scanning modern UUID-tagged detections from the same display name.

-- migrate:up

CREATE INDEX IF NOT EXISTS idx_detections_legacy_stream_label
ON detections(stream_name, label)
WHERE (camera_uuid IS NULL OR camera_uuid = '')
  AND label IS NOT NULL AND TRIM(label) <> '';

-- migrate:down

DROP INDEX IF EXISTS idx_detections_legacy_stream_label;
