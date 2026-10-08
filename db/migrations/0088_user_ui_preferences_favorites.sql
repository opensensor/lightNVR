-- migrate:up
CREATE TABLE IF NOT EXISTS user_preferences (
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    preference_key TEXT NOT NULL,
    value_json TEXT NOT NULL CHECK (json_valid(value_json)),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    PRIMARY KEY (user_id, preference_key)
);

CREATE TABLE IF NOT EXISTS user_favorites (
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    camera_uuid TEXT NOT NULL REFERENCES streams(camera_uuid) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    PRIMARY KEY (user_id, camera_uuid)
);

CREATE INDEX IF NOT EXISTS idx_user_favorites_order
    ON user_favorites(user_id, created_at, camera_uuid);

-- migrate:down
DROP INDEX IF EXISTS idx_user_favorites_order;
DROP TABLE IF EXISTS user_favorites;
DROP TABLE IF EXISTS user_preferences;
