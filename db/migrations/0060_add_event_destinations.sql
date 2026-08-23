-- Managed MQTT destination profiles for normalized event routes.

-- migrate:up

CREATE TABLE event_destinations (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    destination_type TEXT NOT NULL DEFAULT 'mqtt'
        CHECK (destination_type = 'mqtt'),
    broker_host TEXT NOT NULL,
    broker_port INTEGER NOT NULL CHECK (broker_port BETWEEN 1 AND 65535),
    client_id TEXT NOT NULL,
    topic_template TEXT NOT NULL,
    username TEXT NOT NULL DEFAULT '',
    password TEXT NOT NULL DEFAULT '',
    tls_mode TEXT NOT NULL DEFAULT 'system'
        CHECK (tls_mode IN ('disabled', 'system', 'custom_ca', 'mutual')),
    ca_file TEXT NOT NULL DEFAULT '',
    cert_file TEXT NOT NULL DEFAULT '',
    key_file TEXT NOT NULL DEFAULT '',
    keepalive_seconds INTEGER NOT NULL DEFAULT 60
        CHECK (keepalive_seconds BETWEEN 5 AND 3600),
    qos INTEGER NOT NULL DEFAULT 1 CHECK (qos BETWEEN 0 AND 2),
    revision INTEGER NOT NULL DEFAULT 1 CHECK (revision >= 1),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE UNIQUE INDEX idx_event_destinations_name
ON event_destinations(name COLLATE NOCASE);

CREATE UNIQUE INDEX idx_event_destinations_client
ON event_destinations(broker_host COLLATE NOCASE, broker_port, client_id);

CREATE INDEX idx_event_destinations_enabled
ON event_destinations(enabled, destination_type, uuid);

-- migrate:down

DROP INDEX IF EXISTS idx_event_destinations_enabled;
DROP INDEX IF EXISTS idx_event_destinations_client;
DROP INDEX IF EXISTS idx_event_destinations_name;
DROP TABLE IF EXISTS event_destinations;
SELECT 1;
