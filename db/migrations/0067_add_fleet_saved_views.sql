-- Owner-scoped and administrator-published Fleet Explorer views.

-- migrate:up

CREATE TABLE fleet_saved_views (
    uuid TEXT PRIMARY KEY,
    owner_user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL COLLATE NOCASE,
    is_shared INTEGER NOT NULL DEFAULT 0 CHECK (is_shared IN (0, 1)),
    selector_json TEXT NOT NULL,
    search_text TEXT NOT NULL DEFAULT '',
    collection_uuid TEXT,
    columns_json TEXT NOT NULL DEFAULT '[]',
    sort_by TEXT NOT NULL DEFAULT 'name'
        CHECK (sort_by IN (
            'name', 'camera_uuid', 'location', 'health', 'enabled',
            'recording_mode', 'address'
        )),
    sort_order TEXT NOT NULL DEFAULT 'asc'
        CHECK (sort_order IN ('asc', 'desc')),
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE UNIQUE INDEX idx_fleet_saved_views_owner_name
ON fleet_saved_views(COALESCE(owner_user_id, 0), name COLLATE NOCASE);

CREATE INDEX idx_fleet_saved_views_visible
ON fleet_saved_views(is_shared, owner_user_id, name COLLATE NOCASE, uuid);

-- migrate:down

DROP INDEX IF EXISTS idx_fleet_saved_views_visible;
DROP INDEX IF EXISTS idx_fleet_saved_views_owner_name;
DROP TABLE IF EXISTS fleet_saved_views;
