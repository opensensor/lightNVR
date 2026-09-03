-- Keep scoped recording-filter label lookups off the full detections table.

-- migrate:up

CREATE INDEX IF NOT EXISTS idx_detections_stream_label
ON detections(stream_name, label)
WHERE label IS NOT NULL AND TRIM(label) <> '';

-- migrate:down

DROP INDEX IF EXISTS idx_detections_stream_label;
