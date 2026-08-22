-- Hierarchical physical locations and one primary location per camera.

-- migrate:up

CREATE TABLE camera_locations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    parent_uuid TEXT REFERENCES camera_locations(uuid) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    type TEXT NOT NULL DEFAULT 'area',
    sort_order INTEGER NOT NULL DEFAULT 0,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    is_system INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE UNIQUE INDEX idx_camera_locations_sibling_name
ON camera_locations(ifnull(parent_uuid, ''), name COLLATE NOCASE);

CREATE INDEX idx_camera_locations_parent
ON camera_locations(parent_uuid, sort_order, name COLLATE NOCASE);

INSERT INTO camera_locations (uuid, parent_uuid, name, type, is_system)
VALUES (
    lower(
        hex(randomblob(4)) || '-' ||
        hex(randomblob(2)) || '-4' ||
        substr(hex(randomblob(2)), 2) || '-' ||
        substr('89ab', (abs(random()) % 4) + 1, 1) ||
        substr(hex(randomblob(2)), 2) || '-' ||
        hex(randomblob(6))
    ),
    NULL,
    'Unassigned',
    'system',
    1
);

ALTER TABLE streams ADD COLUMN location_uuid TEXT DEFAULT NULL
REFERENCES camera_locations(uuid) ON DELETE RESTRICT;

UPDATE streams
SET location_uuid = (SELECT uuid FROM camera_locations WHERE is_system = 1 LIMIT 1)
WHERE location_uuid IS NULL;

CREATE INDEX idx_streams_location_uuid
ON streams(location_uuid);

-- migrate:down

DROP INDEX IF EXISTS idx_streams_location_uuid;
UPDATE streams SET location_uuid = NULL;
DROP INDEX IF EXISTS idx_camera_locations_parent;
DROP INDEX IF EXISTS idx_camera_locations_sibling_name;
DROP TABLE IF EXISTS camera_locations;
SELECT 1;
