-- Server-backed operator presets for browser-side fisheye ePTZ views.

-- migrate:up

CREATE TABLE eptz_operator_presets (
    uuid TEXT PRIMARY KEY,
    camera_uuid TEXT NOT NULL REFERENCES streams(camera_uuid) ON DELETE CASCADE,
    owner_user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL COLLATE NOCASE,
    is_shared INTEGER NOT NULL DEFAULT 0 CHECK (is_shared IN (0, 1)),
    mode TEXT NOT NULL CHECK (mode IN ('raw', 'dewarp', 'panorama', 'dual')),
    yaw REAL NOT NULL CHECK (yaw BETWEEN -180 AND 180),
    tilt REAL NOT NULL CHECK (tilt BETWEEN -90 AND 30),
    view_fov REAL NOT NULL CHECK (view_fov BETWEEN 20 AND 120),
    secondary_yaw REAL NOT NULL CHECK (secondary_yaw BETWEEN -180 AND 180),
    secondary_tilt REAL NOT NULL CHECK (secondary_tilt BETWEEN -90 AND 30),
    secondary_view_fov REAL NOT NULL CHECK (secondary_view_fov BETWEEN 20 AND 120),
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE UNIQUE INDEX idx_eptz_operator_presets_owner_name
ON eptz_operator_presets(
    camera_uuid, COALESCE(owner_user_id, 0), name COLLATE NOCASE
);

CREATE INDEX idx_eptz_operator_presets_visible
ON eptz_operator_presets(camera_uuid, is_shared, owner_user_id, name COLLATE NOCASE);

-- migrate:down

DROP INDEX IF EXISTS idx_eptz_operator_presets_visible;
DROP INDEX IF EXISTS idx_eptz_operator_presets_owner_name;
DROP TABLE IF EXISTS eptz_operator_presets;
