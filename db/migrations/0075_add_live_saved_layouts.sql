-- Per-user and administrator-shared layouts for the operator Live workspace.

-- migrate:up

CREATE TABLE live_saved_layouts (
    uuid TEXT PRIMARY KEY,
    owner_user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL COLLATE NOCASE,
    is_shared INTEGER NOT NULL DEFAULT 0 CHECK (is_shared IN (0, 1)),
    location_uuid TEXT REFERENCES camera_locations(uuid) ON DELETE SET NULL,
    availability TEXT NOT NULL DEFAULT 'live' CHECK (availability IN (
        'all', 'live', 'offline', 'never_connected', 'disabled'
    )),
    columns INTEGER NOT NULL CHECK (columns BETWEEN 1 AND 9),
    rows INTEGER NOT NULL CHECK (rows BETWEEN 1 AND 9),
    camera_slots_json TEXT NOT NULL DEFAULT '[]',
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CHECK (columns * rows <= 36)
);

CREATE UNIQUE INDEX idx_live_saved_layouts_owner_name
ON live_saved_layouts(COALESCE(owner_user_id, 0), name COLLATE NOCASE);

CREATE INDEX idx_live_saved_layouts_visible
ON live_saved_layouts(is_shared, owner_user_id, name COLLATE NOCASE);

-- migrate:down

DROP INDEX IF EXISTS idx_live_saved_layouts_visible;
DROP INDEX IF EXISTS idx_live_saved_layouts_owner_name;
DROP TABLE IF EXISTS live_saved_layouts;
