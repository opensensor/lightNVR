-- Complete the durable lifecycle mover controls.
--
-- Targets own their archival bandwidth/window policy. Jobs snapshot those
-- values when they are created so an administrator edit cannot change an
-- in-flight transfer. Copy jobs retain the canonical recording and register a
-- separately verified replica.

-- migrate:up

ALTER TABLE storage_targets ADD COLUMN migration_bandwidth_bps INTEGER NOT NULL DEFAULT 0
    CHECK (migration_bandwidth_bps = 0 OR migration_bandwidth_bps >= 65536);
ALTER TABLE storage_targets ADD COLUMN archival_window_start_minute INTEGER NOT NULL DEFAULT 0
    CHECK (archival_window_start_minute BETWEEN 0 AND 1439);
ALTER TABLE storage_targets ADD COLUMN archival_window_end_minute INTEGER NOT NULL DEFAULT 0
    CHECK (archival_window_end_minute BETWEEN 0 AND 1439);
ALTER TABLE storage_targets ADD COLUMN replica_count INTEGER NOT NULL DEFAULT 0
    CHECK (replica_count >= 0);
ALTER TABLE storage_targets ADD COLUMN replica_bytes INTEGER NOT NULL DEFAULT 0
    CHECK (replica_bytes >= 0);

CREATE TABLE storage_migration_jobs_v2 (
    uuid TEXT PRIMARY KEY,
    recording_id INTEGER NOT NULL
        REFERENCES recordings(id) ON DELETE CASCADE,
    owner_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    operation TEXT NOT NULL DEFAULT 'move'
        CHECK (operation IN ('move', 'copy')),
    source_target_uuid TEXT NOT NULL
        REFERENCES storage_targets(uuid) ON DELETE RESTRICT,
    source_object_key TEXT NOT NULL,
    destination_target_uuid TEXT NOT NULL
        REFERENCES storage_targets(uuid) ON DELETE RESTRICT,
    destination_object_key TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'queued'
        CHECK (state IN ('queued', 'copying', 'verifying', 'committing',
                         'cleanup_pending', 'retry_wait', 'completed',
                         'failed', 'cancelled')),
    checksum_mode TEXT NOT NULL DEFAULT 'sha256'
        CHECK (checksum_mode IN ('sha256')),
    checksum TEXT NOT NULL DEFAULT '',
    bytes_total INTEGER NOT NULL DEFAULT 0 CHECK (bytes_total >= 0),
    bytes_copied INTEGER NOT NULL DEFAULT 0 CHECK (bytes_copied >= 0),
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    max_attempts INTEGER NOT NULL DEFAULT 5 CHECK (max_attempts BETWEEN 1 AND 20),
    next_attempt_at INTEGER NOT NULL DEFAULT 0,
    last_error TEXT NOT NULL DEFAULT '',
    cancel_requested INTEGER NOT NULL DEFAULT 0 CHECK (cancel_requested IN (0, 1)),
    bandwidth_limit_bps INTEGER NOT NULL DEFAULT 0
        CHECK (bandwidth_limit_bps = 0 OR bandwidth_limit_bps >= 65536),
    window_start_minute INTEGER NOT NULL DEFAULT 0
        CHECK (window_start_minute BETWEEN 0 AND 1439),
    window_end_minute INTEGER NOT NULL DEFAULT 0
        CHECK (window_end_minute BETWEEN 0 AND 1439),
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    started_at INTEGER,
    completed_at INTEGER,
    CHECK (source_target_uuid <> destination_target_uuid)
);

INSERT INTO storage_migration_jobs_v2(
    uuid, recording_id, owner_user_id, operation, source_target_uuid,
    source_object_key, destination_target_uuid, destination_object_key, state,
    checksum_mode, checksum, bytes_total, bytes_copied, attempt_count,
    max_attempts, next_attempt_at, last_error, revision, created_at, updated_at,
    started_at, completed_at)
SELECT uuid, recording_id, owner_user_id, operation, source_target_uuid,
       source_object_key, destination_target_uuid, destination_object_key, state,
       checksum_mode, checksum, bytes_total, bytes_copied, attempt_count,
       max_attempts, next_attempt_at, last_error, revision, created_at, updated_at,
       started_at, completed_at
FROM storage_migration_jobs;

DROP TABLE storage_migration_jobs;
ALTER TABLE storage_migration_jobs_v2 RENAME TO storage_migration_jobs;

CREATE UNIQUE INDEX idx_storage_migration_one_active_recording
    ON storage_migration_jobs(recording_id)
    WHERE state NOT IN ('completed', 'failed', 'cancelled');
CREATE INDEX idx_storage_migration_due
    ON storage_migration_jobs(state, next_attempt_at, created_at);
CREATE INDEX idx_storage_migration_destination
    ON storage_migration_jobs(destination_target_uuid, state);

CREATE TABLE storage_recording_copies (
    uuid TEXT PRIMARY KEY,
    recording_id INTEGER NOT NULL
        REFERENCES recordings(id) ON DELETE CASCADE,
    target_uuid TEXT NOT NULL
        REFERENCES storage_targets(uuid) ON DELETE RESTRICT,
    object_key TEXT NOT NULL,
    checksum_mode TEXT NOT NULL DEFAULT 'sha256'
        CHECK (checksum_mode IN ('sha256')),
    checksum TEXT NOT NULL,
    size_bytes INTEGER NOT NULL CHECK (size_bytes >= 0),
    verified_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    UNIQUE(recording_id, target_uuid),
    UNIQUE(target_uuid, object_key)
);

CREATE INDEX idx_storage_recording_copies_recording
    ON storage_recording_copies(recording_id);
CREATE INDEX idx_storage_recording_copies_target
    ON storage_recording_copies(target_uuid, recording_id);

CREATE TRIGGER trg_storage_recording_copy_insert
AFTER INSERT ON storage_recording_copies
BEGIN
    UPDATE storage_targets
    SET replica_count = replica_count + 1,
        replica_bytes = replica_bytes + NEW.size_bytes
    WHERE uuid = NEW.target_uuid;
END;

CREATE TRIGGER trg_storage_recording_copy_delete
AFTER DELETE ON storage_recording_copies
BEGIN
    UPDATE storage_targets
    SET replica_count = MAX(replica_count - 1, 0),
        replica_bytes = MAX(replica_bytes - OLD.size_bytes, 0)
    WHERE uuid = OLD.target_uuid;
END;

-- migrate:down
DROP TRIGGER IF EXISTS trg_storage_recording_copy_delete;
DROP TRIGGER IF EXISTS trg_storage_recording_copy_insert;
DROP INDEX IF EXISTS idx_storage_recording_copies_target;
DROP INDEX IF EXISTS idx_storage_recording_copies_recording;
DROP TABLE IF EXISTS storage_recording_copies;

CREATE TABLE storage_migration_jobs_v1 (
    uuid TEXT PRIMARY KEY,
    recording_id INTEGER NOT NULL REFERENCES recordings(id) ON DELETE CASCADE,
    owner_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    operation TEXT NOT NULL DEFAULT 'move' CHECK (operation IN ('move')),
    source_target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,
    source_object_key TEXT NOT NULL,
    destination_target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,
    destination_object_key TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'queued'
        CHECK (state IN ('queued', 'copying', 'verifying', 'committing',
                         'cleanup_pending', 'retry_wait', 'completed',
                         'failed', 'cancelled')),
    checksum_mode TEXT NOT NULL DEFAULT 'sha256' CHECK (checksum_mode IN ('sha256')),
    checksum TEXT NOT NULL DEFAULT '',
    bytes_total INTEGER NOT NULL DEFAULT 0 CHECK (bytes_total >= 0),
    bytes_copied INTEGER NOT NULL DEFAULT 0 CHECK (bytes_copied >= 0),
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    max_attempts INTEGER NOT NULL DEFAULT 5 CHECK (max_attempts BETWEEN 1 AND 20),
    next_attempt_at INTEGER NOT NULL DEFAULT 0,
    last_error TEXT NOT NULL DEFAULT '',
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    started_at INTEGER,
    completed_at INTEGER,
    CHECK (source_target_uuid <> destination_target_uuid)
);
INSERT INTO storage_migration_jobs_v1
SELECT uuid, recording_id, owner_user_id, operation, source_target_uuid,
       source_object_key, destination_target_uuid, destination_object_key, state,
       checksum_mode, checksum, bytes_total, bytes_copied, attempt_count,
       max_attempts, next_attempt_at, last_error, revision, created_at, updated_at,
       started_at, completed_at
FROM storage_migration_jobs WHERE operation = 'move';
DROP TABLE storage_migration_jobs;
ALTER TABLE storage_migration_jobs_v1 RENAME TO storage_migration_jobs;
CREATE UNIQUE INDEX idx_storage_migration_one_active_recording
    ON storage_migration_jobs(recording_id)
    WHERE state NOT IN ('completed', 'failed', 'cancelled');
CREATE INDEX idx_storage_migration_due
    ON storage_migration_jobs(state, next_attempt_at, created_at);
CREATE INDEX idx_storage_migration_destination
    ON storage_migration_jobs(destination_target_uuid, state);

-- Target control columns are intentionally retained on rollback; older code
-- ignores additive columns and preserving them avoids a lossy table rebuild.
