-- Lifecycle scheduling and retention inspect migration state per recording.
-- The active-job uniqueness index excludes terminal jobs and cannot serve
-- predicates such as state <> 'completed', which also inspect failed jobs.
-- Without this lookup index, a large archive backlog causes a full job-table
-- scan for every recording while holding the application's database mutex.
-- migrate:up
CREATE INDEX IF NOT EXISTS idx_storage_migration_recording_state
    ON storage_migration_jobs(recording_id, state);

-- migrate:down
DROP INDEX IF EXISTS idx_storage_migration_recording_state;
