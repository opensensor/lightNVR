-- Durable provider-neutral event delivery outbox.

-- migrate:up

CREATE TABLE event_outbox (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id TEXT NOT NULL,
    event_source TEXT NOT NULL,
    event_type TEXT NOT NULL,
    subject TEXT NOT NULL,
    destination TEXT NOT NULL,
    topic TEXT NOT NULL,
    envelope_json TEXT NOT NULL,
    envelope_bytes INTEGER NOT NULL CHECK (envelope_bytes > 0),
    severity INTEGER NOT NULL CHECK (severity BETWEEN 0 AND 3),
    state TEXT NOT NULL DEFAULT 'pending'
        CHECK (state IN ('pending', 'delivering', 'delivered', 'dead')),
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    next_attempt_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    last_attempt_at INTEGER,
    lease_expires_at INTEGER,
    delivered_at INTEGER,
    dead_at INTEGER,
    last_error TEXT NOT NULL DEFAULT '',
    UNIQUE (event_source, event_id, destination)
);

CREATE INDEX idx_event_outbox_due
ON event_outbox(destination, state, next_attempt_at, severity DESC, id)
WHERE state = 'pending';

CREATE INDEX idx_event_outbox_lease
ON event_outbox(state, lease_expires_at)
WHERE state = 'delivering';

CREATE INDEX idx_event_outbox_expiry
ON event_outbox(state, expires_at);

CREATE INDEX idx_event_outbox_terminal
ON event_outbox(state, updated_at, id)
WHERE state IN ('delivered', 'dead');

-- migrate:down

DROP INDEX IF EXISTS idx_event_outbox_terminal;
DROP INDEX IF EXISTS idx_event_outbox_expiry;
DROP INDEX IF EXISTS idx_event_outbox_lease;
DROP INDEX IF EXISTS idx_event_outbox_due;
DROP TABLE IF EXISTS event_outbox;
SELECT 1;
