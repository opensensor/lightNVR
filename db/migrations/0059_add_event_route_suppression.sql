-- Durable per-route suppression state for normalized event delivery.

-- migrate:up

CREATE TABLE event_route_suppression_state (
    route_uuid TEXT NOT NULL
        REFERENCES event_routes(uuid) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    subject TEXT NOT NULL,
    last_observed_at INTEGER NOT NULL DEFAULT 0
        CHECK (last_observed_at >= 0),
    last_allowed_at INTEGER NOT NULL DEFAULT 0
        CHECK (last_allowed_at >= 0),
    rate_window_started_at INTEGER NOT NULL DEFAULT 0
        CHECK (rate_window_started_at >= 0),
    rate_window_count INTEGER NOT NULL DEFAULT 0
        CHECK (rate_window_count >= 0),
    group_started_at INTEGER NOT NULL DEFAULT 0
        CHECK (group_started_at >= 0),
    suppressed_count INTEGER NOT NULL DEFAULT 0
        CHECK (suppressed_count >= 0),
    last_allowed_event_id TEXT NOT NULL DEFAULT '',
    last_reason TEXT NOT NULL DEFAULT 'allowed'
        CHECK (last_reason IN (
            'allowed', 'debounce', 'cooldown', 'grouping', 'rate'
        )),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    PRIMARY KEY (route_uuid, event_type, subject)
);

CREATE INDEX idx_event_route_suppression_updated
ON event_route_suppression_state(updated_at, route_uuid);

-- migrate:down

DROP INDEX IF EXISTS idx_event_route_suppression_updated;
DROP TABLE IF EXISTS event_route_suppression_state;
SELECT 1;
