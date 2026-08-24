-- Durable, owner-private investigation review bookmarks. A bookmark captures
-- navigation state only; it does not protect or hold recording media.

-- migrate:up

CREATE TABLE investigation_bookmarks (
    uuid TEXT PRIMARY KEY,
    owner_user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    note TEXT NOT NULL DEFAULT '',
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL,
    cursor_time INTEGER NOT NULL,
    primary_camera_uuid TEXT NOT NULL,
    filters_json TEXT NOT NULL DEFAULT '{}',
    representative_result_json TEXT,
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CHECK (end_time > start_time),
    CHECK (cursor_time >= start_time AND cursor_time <= end_time)
);

CREATE INDEX idx_investigation_bookmarks_owner_updated
ON investigation_bookmarks(owner_user_id, updated_at DESC, uuid);

CREATE TABLE investigation_bookmark_cameras (
    bookmark_uuid TEXT NOT NULL
        REFERENCES investigation_bookmarks(uuid) ON DELETE CASCADE,
    camera_uuid TEXT NOT NULL,
    sort_order INTEGER NOT NULL,
    PRIMARY KEY (bookmark_uuid, camera_uuid),
    UNIQUE (bookmark_uuid, sort_order)
);

CREATE INDEX idx_investigation_bookmark_cameras_camera
ON investigation_bookmark_cameras(camera_uuid, bookmark_uuid);

-- migrate:down

DROP INDEX IF EXISTS idx_investigation_bookmark_cameras_camera;
DROP TABLE IF EXISTS investigation_bookmark_cameras;
DROP INDEX IF EXISTS idx_investigation_bookmarks_owner_updated;
DROP TABLE IF EXISTS investigation_bookmarks;
SELECT 1;
