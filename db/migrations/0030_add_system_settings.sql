-- System-wide key/value settings table (used for setup wizard state, etc.)

-- migrate:up

CREATE TABLE IF NOT EXISTS system_settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL DEFAULT '',
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Seed the setup_complete flag so first-run detection works
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('setup_complete', '0');

-- migrate:down

DROP TABLE IF EXISTS system_settings;

