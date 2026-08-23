-- Saved static and selector-backed smart camera collections.

-- migrate:up

CREATE TABLE camera_collections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    collection_type TEXT NOT NULL CHECK (collection_type IN ('static', 'smart')),
    selector_json TEXT NOT NULL DEFAULT '',
    is_shared INTEGER NOT NULL DEFAULT 1,
    owner_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE UNIQUE INDEX idx_camera_collections_name
ON camera_collections(name COLLATE NOCASE);

CREATE INDEX idx_camera_collections_owner
ON camera_collections(owner_user_id, is_shared);

CREATE TABLE camera_collection_members (
    collection_uuid TEXT NOT NULL
        REFERENCES camera_collections(uuid) ON DELETE CASCADE,
    camera_uuid TEXT NOT NULL REFERENCES streams(camera_uuid) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    PRIMARY KEY (collection_uuid, camera_uuid)
);

CREATE INDEX idx_camera_collection_members_camera
ON camera_collection_members(camera_uuid, collection_uuid);

-- migrate:down

DROP INDEX IF EXISTS idx_camera_collection_members_camera;
DROP TABLE IF EXISTS camera_collection_members;
DROP INDEX IF EXISTS idx_camera_collections_owner;
DROP INDEX IF EXISTS idx_camera_collections_name;
DROP TABLE IF EXISTS camera_collections;
SELECT 1;
