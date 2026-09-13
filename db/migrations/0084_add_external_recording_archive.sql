-- External archive targets, lifecycle timing, and durable deletion inventory.
-- Keep the old constrained type column as a compatibility sentinel instead of
-- rebuilding the referenced targets table (which would cascade foreign keys).
-- migrate:up
ALTER TABLE storage_targets RENAME COLUMN target_type TO legacy_target_type;
ALTER TABLE storage_targets ADD COLUMN target_type TEXT NOT NULL DEFAULT 'filesystem'
    CHECK (target_type IN ('filesystem','s3'));
ALTER TABLE storage_targets ADD COLUMN endpoint TEXT NOT NULL DEFAULT '';
ALTER TABLE storage_targets ADD COLUMN region TEXT NOT NULL DEFAULT '';
ALTER TABLE storage_targets ADD COLUMN bucket TEXT NOT NULL DEFAULT '';
ALTER TABLE storage_targets ADD COLUMN credential_ref TEXT NOT NULL DEFAULT '';
ALTER TABLE storage_targets ADD COLUMN archive_budget_bytes INTEGER NOT NULL DEFAULT 0
    CHECK (archive_budget_bytes >= 0);

ALTER TABLE storage_policies ADD COLUMN archive_after_seconds INTEGER NOT NULL DEFAULT -1
    CHECK (archive_after_seconds BETWEEN -1 AND 2147483647);
ALTER TABLE storage_policies ADD COLUMN hot_residency_seconds INTEGER NOT NULL DEFAULT -1
    CHECK (hot_residency_seconds BETWEEN -1 AND 2147483647);
ALTER TABLE storage_policies ADD COLUMN archive_protected INTEGER NOT NULL DEFAULT 0
    CHECK (archive_protected IN (0,1));
ALTER TABLE storage_policies ADD COLUMN archive_on_pressure INTEGER NOT NULL DEFAULT 0
    CHECK (archive_on_pressure IN (0,1));

ALTER TABLE recordings ADD COLUMN storage_policy_uuid TEXT;
UPDATE recordings SET storage_policy_uuid=substr(placement_reason,instr(placement_reason,':')+1)
    WHERE placement_reason LIKE 'policy-%';
CREATE TRIGGER trg_recording_policy_identity AFTER INSERT ON recordings
WHEN NEW.storage_policy_uuid IS NULL AND NEW.placement_reason LIKE 'policy-%'
BEGIN
    UPDATE recordings SET storage_policy_uuid=substr(NEW.placement_reason,instr(NEW.placement_reason,':')+1)
        WHERE id=NEW.id;
END;
ALTER TABLE recordings ADD COLUMN archive_checksum TEXT NOT NULL DEFAULT '';
ALTER TABLE recordings ADD COLUMN deletion_pending INTEGER NOT NULL DEFAULT 0
    CHECK (deletion_pending IN (0,1));

CREATE TABLE storage_deletions (
    uuid TEXT PRIMARY KEY,
    recording_id INTEGER NOT NULL,
    camera_uuid TEXT NOT NULL DEFAULT '',
    stream_name TEXT NOT NULL DEFAULT '',
    reason TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    completed_at INTEGER
);
CREATE TABLE storage_deletion_objects (
    id INTEGER PRIMARY KEY,
    deletion_uuid TEXT NOT NULL REFERENCES storage_deletions(uuid) ON DELETE CASCADE,
    target_uuid TEXT REFERENCES storage_targets(uuid) ON DELETE RESTRICT,
    original_target_uuid TEXT NOT NULL DEFAULT '',
    upload_id TEXT NOT NULL DEFAULT '',
    object_key TEXT NOT NULL DEFAULT '',
    file_path TEXT NOT NULL DEFAULT '',
    size_bytes INTEGER NOT NULL DEFAULT 0,
    state TEXT NOT NULL DEFAULT 'pending' CHECK(state IN ('pending','retry_wait','completed')),
    attempt_count INTEGER NOT NULL DEFAULT 0,
    next_attempt_at INTEGER NOT NULL DEFAULT 0,
    last_error TEXT NOT NULL DEFAULT '',
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now'))
);
CREATE TRIGGER trg_deletion_target_identity AFTER INSERT ON storage_deletion_objects
BEGIN
    UPDATE storage_deletion_objects SET original_target_uuid=COALESCE(NEW.target_uuid,'') WHERE id=NEW.id;
END;
CREATE INDEX idx_storage_deletion_due ON storage_deletion_objects(state,next_attempt_at,id);
CREATE INDEX idx_storage_deletion_recording ON storage_deletions(recording_id);

ALTER TABLE storage_migration_jobs ADD COLUMN upload_id TEXT NOT NULL DEFAULT '';
ALTER TABLE storage_migration_jobs ADD COLUMN upload_parts TEXT NOT NULL DEFAULT '';
ALTER TABLE storage_migration_jobs ADD COLUMN artifacts_cleaned INTEGER NOT NULL DEFAULT 0 CHECK(artifacts_cleaned IN(0,1));

CREATE TABLE storage_retrieval_jobs (
    recording_id INTEGER PRIMARY KEY REFERENCES recordings(id) ON DELETE CASCADE,
    target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,
    object_key TEXT NOT NULL,
    checksum TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'queued' CHECK(state IN ('queued','fetching','ready','failed')),
    last_access_at INTEGER NOT NULL DEFAULT (strftime('%s','now')),
    next_attempt_at INTEGER NOT NULL DEFAULT 0,
    last_error TEXT NOT NULL DEFAULT ''
);
CREATE TABLE storage_read_leases (
    recording_id INTEGER PRIMARY KEY REFERENCES recordings(id) ON DELETE CASCADE,
    expires_at INTEGER NOT NULL
);

-- Retain the policy revision selected at capture. Legacy unattributed rows
-- continue using the existing stream/system rules.
CREATE TABLE storage_policy_versions AS SELECT * FROM storage_policies;
CREATE UNIQUE INDEX idx_storage_policy_version ON storage_policy_versions(uuid,revision);
CREATE TRIGGER trg_storage_policy_snapshot_insert AFTER INSERT ON storage_policies
BEGIN
    INSERT INTO storage_policy_versions SELECT * FROM storage_policies WHERE uuid=NEW.uuid;
END;
CREATE TRIGGER trg_storage_policy_snapshot_update AFTER UPDATE ON storage_policies
WHEN NEW.revision<>OLD.revision
BEGIN
    INSERT INTO storage_policy_versions SELECT * FROM storage_policies WHERE uuid=NEW.uuid;
END;
CREATE TRIGGER trg_storage_policy_snapshot_delete AFTER DELETE ON storage_policies
BEGIN
    DELETE FROM storage_policy_versions WHERE uuid=OLD.uuid AND NOT EXISTS(SELECT 1 FROM recordings r
        WHERE r.storage_policy_uuid=storage_policy_versions.uuid AND r.storage_policy_version=storage_policy_versions.revision);
END;
UPDATE recordings SET storage_policy_version=(SELECT revision FROM storage_policies p
    WHERE p.uuid=recordings.storage_policy_uuid) WHERE storage_policy_uuid IN(SELECT uuid FROM storage_policies);
CREATE VIEW storage_recording_policies AS
SELECT r.id AS recording_id,v.* FROM recordings r JOIN storage_policy_versions v
    ON v.uuid=r.storage_policy_uuid AND v.revision=r.storage_policy_version
UNION ALL
SELECT r.id AS recording_id,p.* FROM recordings r JOIN storage_policies p
    ON p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1))
    WHERE NOT EXISTS(SELECT 1 FROM storage_policy_versions v WHERE v.uuid=r.storage_policy_uuid
        AND v.revision=r.storage_policy_version);

-- migrate:down
-- Retain additive metadata and deletion inventory: removing it would orphan media.
SELECT 1;
