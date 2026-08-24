-- Selector-driven recording placement and mount-loss protection.

-- migrate:up

ALTER TABLE storage_targets ADD COLUMN mount_required INTEGER NOT NULL DEFAULT 0
    CHECK (mount_required IN (0, 1));
ALTER TABLE storage_targets ADD COLUMN mount_guard_path TEXT NOT NULL DEFAULT '';

CREATE TABLE IF NOT EXISTS storage_policies (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL COLLATE NOCASE,
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    priority INTEGER NOT NULL DEFAULT 100,
    selector_json TEXT NOT NULL,
    primary_target_uuid TEXT NOT NULL
        REFERENCES storage_targets(uuid) ON DELETE RESTRICT,
    fallback_mode TEXT NOT NULL DEFAULT 'default'
        CHECK (fallback_mode IN ('default', 'target', 'pause', 'fail')),
    fallback_target_uuid TEXT
        REFERENCES storage_targets(uuid) ON DELETE RESTRICT,
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CHECK (
        (fallback_mode = 'target' AND fallback_target_uuid IS NOT NULL) OR
        (fallback_mode != 'target' AND fallback_target_uuid IS NULL)
    ),
    CHECK (fallback_target_uuid IS NULL OR
           fallback_target_uuid != primary_target_uuid)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_storage_policies_name
    ON storage_policies(name COLLATE NOCASE);
CREATE INDEX IF NOT EXISTS idx_storage_policies_evaluation
    ON storage_policies(enabled, priority DESC, name COLLATE NOCASE, uuid);

-- migrate:down
DROP INDEX IF EXISTS idx_storage_policies_evaluation;
DROP INDEX IF EXISTS idx_storage_policies_name;
DROP TABLE IF EXISTS storage_policies;
