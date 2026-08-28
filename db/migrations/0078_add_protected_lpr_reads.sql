-- Protected, independently retained license-plate recognition reads.

-- migrate:up

CREATE TABLE lpr_reads (
    uuid TEXT PRIMARY KEY,
    camera_uuid TEXT NOT NULL,
    stream_name TEXT NOT NULL,
    observed_at_ms INTEGER NOT NULL,
    received_at_ms INTEGER NOT NULL,
    source TEXT NOT NULL CHECK (source IN (
        'onvif_profile_m', 'onvif_vendor', 'vendor_api', 'metadata_track'
    )),
    vendor_topic TEXT NOT NULL DEFAULT '',
    plate_nonce BLOB NOT NULL CHECK (length(plate_nonce) = 12),
    plate_ciphertext BLOB NOT NULL,
    plate_tag BLOB NOT NULL CHECK (length(plate_tag) = 16),
    plate_exact_hmac BLOB NOT NULL CHECK (length(plate_exact_hmac) = 32),
    dedupe_hmac BLOB NOT NULL CHECK (length(dedupe_hmac) = 32),
    confidence REAL CHECK (confidence IS NULL OR (confidence >= 0.0 AND confidence <= 1.0)),
    country TEXT,
    region TEXT,
    plate_type TEXT,
    direction TEXT,
    lane TEXT,
    vehicle_type TEXT,
    vehicle_color TEXT,
    object_id TEXT,
    correlation_id TEXT,
    bbox_left REAL,
    bbox_top REAL,
    bbox_right REAL,
    bbox_bottom REAL,
    recording_id INTEGER REFERENCES recordings(id) ON DELETE SET NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(dedupe_hmac)
);

CREATE INDEX idx_lpr_reads_camera_time
ON lpr_reads(camera_uuid, observed_at_ms DESC);

CREATE INDEX idx_lpr_reads_exact_time
ON lpr_reads(plate_exact_hmac, observed_at_ms DESC);

CREATE INDEX idx_lpr_reads_retention
ON lpr_reads(received_at_ms);

INSERT OR IGNORE INTO system_settings (key, value)
VALUES ('lpr_retention_days', '30');

-- New persisted action-mask positions are appended; existing positions never
-- move. Built-in roles fail closed: only Administrator receives these actions.
INSERT INTO authz_actions
    (action_key, category, description, camera_scoped, destructive, sort_order, bit_index)
VALUES
    ('lpr.read', 'License plates', 'View protected plate values', 1, 0, 160, 15),
    ('lpr.search', 'License plates', 'Search protected plate reads', 1, 0, 170, 16),
    ('lpr.export', 'License plates', 'Export protected plate reads', 1, 0, 180, 17),
    ('lpr.delete', 'License plates', 'Permanently delete protected plate reads', 1, 1, 190, 18);

INSERT INTO authz_role_actions (role_uuid, action_key)
SELECT '00000000-0000-4000-8000-000000000001', action_key
FROM authz_actions
WHERE action_key IN ('lpr.read', 'lpr.search', 'lpr.export', 'lpr.delete');

-- migrate:down

DELETE FROM authz_role_actions
WHERE action_key IN ('lpr.read', 'lpr.search', 'lpr.export', 'lpr.delete');
DELETE FROM authz_actions
WHERE action_key IN ('lpr.read', 'lpr.search', 'lpr.export', 'lpr.delete');
DELETE FROM system_settings WHERE key = 'lpr_retention_days';
DROP INDEX IF EXISTS idx_lpr_reads_retention;
DROP INDEX IF EXISTS idx_lpr_reads_exact_time;
DROP INDEX IF EXISTS idx_lpr_reads_camera_time;
DROP TABLE IF EXISTS lpr_reads;
