-- INT_01 durable object observations, inference-frame evidence assets, the
-- monotonic change log behind the history cursor, and transactional
-- publication staging for the event relay. Normative contract:
-- docs/integrations/OBSERVATIONS_CONTRACT.md (sections 3-5, 7, 13, 14).
--
-- Every observation references an asset row (contract D5); observations with
-- no media reference an 'unavailable' asset. Reason values stay open text so a
-- new reason is a minor contract change, but the reason must be NULL exactly
-- when the state is pending or ready. Provisional budget bounds (section 13)
-- are enforced by the API, not by CHECKs, so tuning them needs no rebuild.

-- migrate:up

CREATE TABLE observation_assets (
    uuid TEXT PRIMARY KEY,
    camera_uuid TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('pending', 'ready', 'unavailable', 'expired')),
    reason TEXT CHECK (reason IS NULL OR length(reason) BETWEEN 1 AND 32),
    origin TEXT CHECK (origin IS NULL OR origin IN ('inference_input', 'submitted_image')),
    media_type TEXT,
    path TEXT,
    bytes INTEGER CHECK (bytes IS NULL OR bytes >= 0),
    width INTEGER CHECK (width IS NULL OR width > 0),
    height INTEGER CHECK (height IS NULL OR height > 0),
    captured_at_ms INTEGER,
    transform_json TEXT,
    protected INTEGER NOT NULL DEFAULT 0 CHECK (protected IN (0, 1)),
    expires_at_ms INTEGER,
    created_at_ms INTEGER NOT NULL,
    updated_at_ms INTEGER NOT NULL,
    CHECK ((state IN ('pending', 'ready')) = (reason IS NULL))
);

-- Reconciliation scans by state; retention walks one camera's ready assets
-- oldest first.
CREATE INDEX idx_observation_assets_state_camera
ON observation_assets(state, camera_uuid, created_at_ms);

CREATE TABLE object_observations (
    uuid TEXT PRIMARY KEY,
    camera_uuid TEXT NOT NULL,
    stream_name_at_capture TEXT NOT NULL,
    batch_id TEXT NOT NULL,
    object_index INTEGER NOT NULL CHECK (object_index BETWEEN 0 AND 19),
    observed_at_ms INTEGER NOT NULL,
    created_at_ms INTEGER NOT NULL,
    engine_key TEXT NOT NULL,
    engine_type TEXT NOT NULL CHECK (engine_type IN ('object', 'api', 'onvif', 'external')),
    model TEXT,
    model_version TEXT,
    label TEXT NOT NULL CHECK (length(label) BETWEEN 1 AND 31),
    confidence REAL NOT NULL CHECK (confidence >= 0.0 AND confidence <= 1.0),
    box_x REAL,
    box_y REAL,
    box_width REAL,
    box_height REAL,
    track_id TEXT,
    track_namespace TEXT,
    track_session TEXT,
    recording_id INTEGER REFERENCES recordings(id) ON DELETE SET NULL,
    asset_uuid TEXT REFERENCES observation_assets(uuid),
    revision INTEGER NOT NULL DEFAULT 1 CHECK (revision >= 1),
    UNIQUE(batch_id, object_index),
    -- Normalized xywh relative to the stored evidence image; all or nothing.
    -- The 1e-6 slack absorbs rounding to 6 decimals. IS NOT NULL is explicit
    -- because a CHECK that evaluates to NULL passes.
    CHECK ((box_x IS NULL AND box_y IS NULL AND box_width IS NULL AND box_height IS NULL)
        OR (box_x IS NOT NULL AND box_y IS NOT NULL
            AND box_width IS NOT NULL AND box_height IS NOT NULL
            AND box_x >= 0.0 AND box_y >= 0.0
            AND box_width > 0.0 AND box_height > 0.0
            AND box_x + box_width <= 1.000001 AND box_y + box_height <= 1.000001)),
    CHECK ((track_id IS NULL AND track_namespace IS NULL AND track_session IS NULL)
        OR (length(track_id) BETWEEN 1 AND 64
            AND track_namespace IS NOT NULL AND track_session IS NOT NULL))
);

CREATE INDEX idx_object_observations_camera_time
ON object_observations(camera_uuid, observed_at_ms);

CREATE INDEX idx_object_observations_label_time
ON object_observations(label, observed_at_ms);

CREATE INDEX idx_object_observations_created
ON object_observations(created_at_ms);

-- Asset transitions bump every referencing observation; refcount is derived.
CREATE INDEX idx_object_observations_asset
ON object_observations(asset_uuid);

-- Keeps the ON DELETE SET NULL action cheap on every recording delete.
CREATE INDEX idx_object_observations_recording
ON object_observations(recording_id);

-- seq is the history cursor: monotonic persistence order, never reused.
CREATE TABLE observation_changes (
    seq INTEGER PRIMARY KEY AUTOINCREMENT,
    observation_uuid TEXT NOT NULL REFERENCES object_observations(uuid) ON DELETE CASCADE,
    revision INTEGER NOT NULL CHECK (revision >= 1),
    cause TEXT NOT NULL CHECK (cause IN ('created', 'evidence', 'classification')),
    evidence_state TEXT NOT NULL CHECK (evidence_state IN ('pending', 'ready', 'unavailable', 'expired')),
    evidence_reason TEXT CHECK (evidence_reason IS NULL OR length(evidence_reason) BETWEEN 1 AND 32),
    created_at_ms INTEGER NOT NULL,
    UNIQUE(observation_uuid, revision),
    CHECK ((evidence_state IN ('pending', 'ready')) = (evidence_reason IS NULL)),
    CHECK ((cause = 'created') = (revision = 1))
);

CREATE TABLE observation_publications (
    change_seq INTEGER PRIMARY KEY REFERENCES observation_changes(seq) ON DELETE CASCADE,
    message_id TEXT NOT NULL UNIQUE,
    state TEXT NOT NULL DEFAULT 'pending' CHECK (state IN (
        'pending', 'relayed', 'relayed_no_route', 'expired', 'failed'
    )),
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    last_error TEXT,
    updated_at_ms INTEGER NOT NULL
);

-- The relay reads pending rows in seq order.
CREATE INDEX idx_observation_publications_pending
ON observation_publications(change_seq) WHERE state = 'pending';

-- One row. epoch_uuid identifies this DB generation for cursors (contract
-- D9); oldest_seq is the lowest seq that may still be retained (prune
-- boundary, initially 1). Kept out of system_settings so it can be written
-- inside observation transactions.
CREATE TABLE observation_meta (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    epoch_uuid TEXT NOT NULL CHECK (length(epoch_uuid) = 36),
    oldest_seq INTEGER NOT NULL DEFAULT 1 CHECK (oldest_seq >= 1)
);

INSERT INTO observation_meta (id, epoch_uuid, oldest_seq)
VALUES (
    1,
    lower(
        hex(randomblob(4)) || '-' ||
        hex(randomblob(2)) || '-4' ||
        substr(hex(randomblob(2)), 2) || '-' ||
        substr('89ab', (abs(random()) % 4) + 1, 1) ||
        substr(hex(randomblob(2)), 2) || '-' ||
        hex(randomblob(6))
    ),
    1
);

-- Per-camera evidence settings (contract section 13). Evidence is off by
-- default; an empty label list means every object label.
ALTER TABLE streams ADD COLUMN observation_evidence_enabled INTEGER NOT NULL DEFAULT 0
    CHECK (observation_evidence_enabled IN (0, 1));
ALTER TABLE streams ADD COLUMN observation_labels TEXT NOT NULL DEFAULT '';
ALTER TABLE streams ADD COLUMN observation_max_age_days INTEGER NOT NULL DEFAULT 7
    CHECK (observation_max_age_days > 0);
ALTER TABLE streams ADD COLUMN observation_max_bytes INTEGER NOT NULL DEFAULT 2147483648
    CHECK (observation_max_bytes > 0);

INSERT OR IGNORE INTO system_settings (key, value)
VALUES ('observation_history_retention_days', '30');

INSERT OR IGNORE INTO system_settings (key, value)
VALUES ('observation_global_queue_bytes', '33554432');

-- migrate:down

-- Drops all observation history and evidence metadata; take a backup first.
-- Evidence files under <storage_path>/observations are left on disk.
DELETE FROM system_settings WHERE key IN (
    'observation_history_retention_days', 'observation_global_queue_bytes'
);
ALTER TABLE streams DROP COLUMN observation_max_bytes;
ALTER TABLE streams DROP COLUMN observation_max_age_days;
ALTER TABLE streams DROP COLUMN observation_labels;
ALTER TABLE streams DROP COLUMN observation_evidence_enabled;
DROP TABLE IF EXISTS observation_meta;
DROP INDEX IF EXISTS idx_observation_publications_pending;
DROP TABLE IF EXISTS observation_publications;
DROP TABLE IF EXISTS observation_changes;
DROP INDEX IF EXISTS idx_object_observations_recording;
DROP INDEX IF EXISTS idx_object_observations_asset;
DROP INDEX IF EXISTS idx_object_observations_created;
DROP INDEX IF EXISTS idx_object_observations_label_time;
DROP INDEX IF EXISTS idx_object_observations_camera_time;
DROP TABLE IF EXISTS object_observations;
DROP INDEX IF EXISTS idx_observation_assets_state_camera;
DROP TABLE IF EXISTS observation_assets;
