-- Per-user favorite cameras, surfaced as a filter on the Live view.
-- A NULL user_id holds the installation-wide set used while authentication is
-- disabled, mirroring live_saved_layouts.

-- migrate:up

CREATE TABLE camera_favorites (
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    camera_uuid TEXT NOT NULL REFERENCES streams(camera_uuid) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE UNIQUE INDEX idx_camera_favorites_user_camera
ON camera_favorites(COALESCE(user_id, 0), camera_uuid);

CREATE INDEX idx_camera_favorites_camera
ON camera_favorites(camera_uuid);

-- migrate:down

DROP INDEX IF EXISTS idx_camera_favorites_camera;
DROP INDEX IF EXISTS idx_camera_favorites_user_camera;
DROP TABLE IF EXISTS camera_favorites;
