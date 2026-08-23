-- Append-only security and sensitive-operation audit history.

-- migrate:up

CREATE TABLE audit_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    occurred_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    request_id TEXT NOT NULL,
    principal_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    principal_username TEXT NOT NULL DEFAULT '',
    auth_method TEXT NOT NULL DEFAULT 'unknown',
    api_token_uuid TEXT,
    action TEXT NOT NULL,
    target_type TEXT,
    target_uuid TEXT,
    outcome TEXT NOT NULL CHECK (outcome IN ('allowed', 'denied', 'success', 'failure', 'error')),
    remote_address TEXT,
    details_json TEXT NOT NULL DEFAULT '{}'
);

CREATE INDEX idx_audit_events_occurred
ON audit_events(occurred_at DESC, id DESC);

CREATE INDEX idx_audit_events_principal
ON audit_events(principal_user_id, occurred_at DESC);

CREATE INDEX idx_audit_events_action_outcome
ON audit_events(action, outcome, occurred_at DESC);

CREATE INDEX idx_audit_events_target
ON audit_events(target_uuid, occurred_at DESC)
WHERE target_uuid IS NOT NULL;

INSERT OR IGNORE INTO system_settings (key, value)
VALUES ('audit_retention_days', '365');

-- migrate:down

DELETE FROM system_settings WHERE key = 'audit_retention_days';
DROP INDEX IF EXISTS idx_audit_events_target;
DROP INDEX IF EXISTS idx_audit_events_action_outcome;
DROP INDEX IF EXISTS idx_audit_events_principal;
DROP INDEX IF EXISTS idx_audit_events_occurred;
DROP TABLE IF EXISTS audit_events;
SELECT 1;
