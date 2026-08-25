-- Durable recording migration jobs.
--
-- A job snapshots the source and destination identities so a policy edit or
-- process restart cannot silently redirect an in-flight copy. The recording
-- row is changed only after the destination has been independently verified.

-- migrate:up

CREATE TABLE IF NOT EXISTS storage_migration_jobs (
    uuid TEXT PRIMARY KEY,
    recording_id INTEGER NOT NULL
        REFERENCES recordings(id) ON DELETE CASCADE,
    owner_user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    operation TEXT NOT NULL DEFAULT 'move'
        CHECK (operation IN ('move')),
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
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    started_at INTEGER,
    completed_at INTEGER,
    CHECK (source_target_uuid <> destination_target_uuid)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_storage_migration_one_active_recording
    ON storage_migration_jobs(recording_id)
    WHERE state NOT IN ('completed', 'failed', 'cancelled');
CREATE INDEX IF NOT EXISTS idx_storage_migration_due
    ON storage_migration_jobs(state, next_attempt_at, created_at);
CREATE INDEX IF NOT EXISTS idx_storage_migration_destination
    ON storage_migration_jobs(destination_target_uuid, state);

-- migrate:down
DROP INDEX IF EXISTS idx_storage_migration_destination;
DROP INDEX IF EXISTS idx_storage_migration_due;
DROP INDEX IF EXISTS idx_storage_migration_one_active_recording;
DROP TABLE IF EXISTS storage_migration_jobs;
