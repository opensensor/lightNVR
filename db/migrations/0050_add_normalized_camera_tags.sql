-- Stable camera tag identities and normalized many-to-many assignments.

-- migrate:up

CREATE TABLE camera_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    label TEXT NOT NULL,
    color TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE UNIQUE INDEX idx_camera_tags_label
ON camera_tags(label COLLATE NOCASE);

CREATE TABLE camera_tag_assignments (
    camera_uuid TEXT NOT NULL REFERENCES streams(camera_uuid) ON DELETE CASCADE,
    tag_uuid TEXT NOT NULL REFERENCES camera_tags(uuid) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    PRIMARY KEY (camera_uuid, tag_uuid)
);

CREATE INDEX idx_camera_tag_assignments_tag
ON camera_tag_assignments(tag_uuid, camera_uuid);

-- Legacy streams.tags values are backfilled by db_camera_tags_backfill_legacy()
-- immediately after migrations. The C backfill handles whitespace and arbitrary
-- tag text without depending on optional SQLite JSON extensions.

-- migrate:down

DROP INDEX IF EXISTS idx_camera_tag_assignments_tag;
DROP TABLE IF EXISTS camera_tag_assignments;
DROP INDEX IF EXISTS idx_camera_tags_label;
DROP TABLE IF EXISTS camera_tags;
SELECT 1;
