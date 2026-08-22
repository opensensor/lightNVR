-- Stable camera identity for fleet organization and policy bindings.
-- 0047 is reserved by recording-history query optimization work.

-- migrate:up

ALTER TABLE streams ADD COLUMN camera_uuid TEXT NOT NULL DEFAULT '';

UPDATE streams
SET camera_uuid = lower(
    hex(randomblob(4)) || '-' ||
    hex(randomblob(2)) || '-4' ||
    substr(hex(randomblob(2)), 2) || '-' ||
    substr('89ab', (abs(random()) % 4) + 1, 1) ||
    substr(hex(randomblob(2)), 2) || '-' ||
    hex(randomblob(6))
)
WHERE camera_uuid = '';

CREATE UNIQUE INDEX IF NOT EXISTS idx_streams_camera_uuid
ON streams(camera_uuid);

-- migrate:down

DROP INDEX IF EXISTS idx_streams_camera_uuid;
SELECT 1;
