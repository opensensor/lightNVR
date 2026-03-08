-- Add sliding-session and trusted-device support for web authentication.

-- migrate:up

ALTER TABLE sessions ADD COLUMN last_activity_at INTEGER;
UPDATE sessions
   SET last_activity_at = COALESCE(last_activity_at, created_at, strftime('%s', 'now'));

ALTER TABLE sessions ADD COLUMN idle_expires_at INTEGER;
UPDATE sessions
   SET idle_expires_at = COALESCE(idle_expires_at, expires_at, strftime('%s', 'now'));

CREATE INDEX IF NOT EXISTS idx_sessions_user_last_activity
    ON sessions(user_id, last_activity_at DESC);

CREATE TABLE IF NOT EXISTS trusted_devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL UNIQUE,
    ip_address TEXT,
    user_agent TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    last_used_at INTEGER,
    expires_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_trusted_devices_user_id ON trusted_devices(user_id);
CREATE INDEX IF NOT EXISTS idx_trusted_devices_token ON trusted_devices(token);
CREATE INDEX IF NOT EXISTS idx_trusted_devices_expires_at ON trusted_devices(expires_at);

-- migrate:down

DROP INDEX IF EXISTS idx_trusted_devices_expires_at;
DROP INDEX IF EXISTS idx_trusted_devices_token;
DROP INDEX IF EXISTS idx_trusted_devices_user_id;
DROP TABLE IF EXISTS trusted_devices;
DROP INDEX IF EXISTS idx_sessions_user_last_activity;
