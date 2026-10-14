-- Keep the small policy tables outside the recording-history loops, including
-- when callers materialize this UNION ALL view. CROSS JOIN fixes loop order
-- in SQLite so empty policy tables never scan recordings.
-- migrate:up
CREATE INDEX idx_recordings_storage_policy_revision
    ON recordings(storage_policy_uuid, storage_policy_version);
DROP VIEW storage_recording_policies;
CREATE VIEW storage_recording_policies AS
SELECT r.id AS recording_id,v.* FROM storage_policy_versions v CROSS JOIN recordings r
    ON v.uuid=r.storage_policy_uuid AND v.revision=r.storage_policy_version
UNION ALL
SELECT r.id AS recording_id,p.* FROM storage_policies p CROSS JOIN recordings r
    ON p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1))
    WHERE NOT EXISTS(SELECT 1 FROM storage_policy_versions v WHERE v.uuid=r.storage_policy_uuid
        AND v.revision=r.storage_policy_version);

-- migrate:down
DROP VIEW storage_recording_policies;
CREATE VIEW storage_recording_policies AS
SELECT r.id AS recording_id,v.* FROM recordings r JOIN storage_policy_versions v
    ON v.uuid=r.storage_policy_uuid AND v.revision=r.storage_policy_version
UNION ALL
SELECT r.id AS recording_id,p.* FROM recordings r JOIN storage_policies p
    ON p.uuid=COALESCE(r.storage_policy_uuid,substr(r.placement_reason,instr(r.placement_reason,':')+1))
    WHERE NOT EXISTS(SELECT 1 FROM storage_policy_versions v WHERE v.uuid=r.storage_policy_uuid
        AND v.revision=r.storage_policy_version);
DROP INDEX idx_recordings_storage_policy_revision;
