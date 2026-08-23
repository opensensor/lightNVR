-- Versioned event route definitions with camera scope and suppression policy.

-- migrate:up

CREATE TABLE event_routes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    destination_key TEXT NOT NULL DEFAULT 'mqtt:default',
    scope_type TEXT NOT NULL DEFAULT 'all'
        CHECK (scope_type IN ('all', 'selector')),
    selector_json TEXT,
    predicate_json TEXT NOT NULL DEFAULT '{"version":1}',
    schedule_json TEXT NOT NULL DEFAULT
        '{"version":1,"timezone":"UTC","windows":[]}',
    debounce_seconds INTEGER NOT NULL DEFAULT 0
        CHECK (debounce_seconds BETWEEN 0 AND 86400),
    cooldown_seconds INTEGER NOT NULL DEFAULT 0
        CHECK (cooldown_seconds BETWEEN 0 AND 604800),
    grouping_window_seconds INTEGER NOT NULL DEFAULT 0
        CHECK (grouping_window_seconds BETWEEN 0 AND 3600),
    max_events_per_minute INTEGER NOT NULL DEFAULT 0
        CHECK (max_events_per_minute BETWEEN 0 AND 60000),
    revision INTEGER NOT NULL DEFAULT 1 CHECK (revision >= 1),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CHECK (
        (scope_type = 'all' AND selector_json IS NULL) OR
        (scope_type = 'selector' AND selector_json IS NOT NULL)
    )
);

CREATE UNIQUE INDEX idx_event_routes_name
ON event_routes(name COLLATE NOCASE);

CREATE INDEX idx_event_routes_delivery
ON event_routes(enabled, destination_key, uuid);

CREATE TABLE event_route_types (
    route_uuid TEXT NOT NULL REFERENCES event_routes(uuid) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    PRIMARY KEY (route_uuid, event_type)
);

CREATE INDEX idx_event_route_types_type
ON event_route_types(event_type, route_uuid);

-- migrate:down

DROP INDEX IF EXISTS idx_event_route_types_type;
DROP TABLE IF EXISTS event_route_types;
DROP INDEX IF EXISTS idx_event_routes_delivery;
DROP INDEX IF EXISTS idx_event_routes_name;
DROP TABLE IF EXISTS event_routes;
SELECT 1;
