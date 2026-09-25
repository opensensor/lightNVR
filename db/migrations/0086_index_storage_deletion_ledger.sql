-- Retention deletes one recording at a time through the durable deletion
-- ledger (storage_deletions / storage_deletion_objects). Finalizing a deletion
-- and the worker's due-object scan used to walk the whole ledger and the whole
-- recordings table on every call: nothing indexed deletion_uuid, completed_at
-- or deletion_pending, and idx_storage_deletion_due(state,...) cannot serve
-- the state <> 'completed' predicate. On a large catalog that cost ~0.5 s per
-- recording under the database mutex, so retention fell behind ingestion.
--
-- deletion_uuid also backs the ON DELETE CASCADE from storage_deletions to its
-- objects, which storage_deletion_prune_completed relies on together with the
-- completed_at index to trim finished ledger rows in bounded batches.
-- migrate:up
CREATE INDEX IF NOT EXISTS idx_recordings_deletion_pending
    ON recordings(deletion_pending) WHERE deletion_pending = 1;
CREATE INDEX IF NOT EXISTS idx_storage_deletion_object_deletion
    ON storage_deletion_objects(deletion_uuid);
CREATE INDEX IF NOT EXISTS idx_storage_deletion_object_open
    ON storage_deletion_objects(next_attempt_at, id) WHERE state <> 'completed';
CREATE INDEX IF NOT EXISTS idx_storage_deletion_completed
    ON storage_deletions(completed_at);

-- migrate:down
DROP INDEX IF EXISTS idx_storage_deletion_completed;
DROP INDEX IF EXISTS idx_storage_deletion_object_open;
DROP INDEX IF EXISTS idx_storage_deletion_object_deletion;
DROP INDEX IF EXISTS idx_recordings_deletion_pending;
