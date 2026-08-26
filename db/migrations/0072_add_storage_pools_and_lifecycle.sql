-- Storage pools, lifecycle intent, and persistent compliance conditions.

-- migrate:up

CREATE TABLE storage_pools (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE,
    strategy TEXT NOT NULL DEFAULT 'most_free'
        CHECK (strategy IN ('most_free', 'round_robin', 'priority')),
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    allocation_cursor INTEGER NOT NULL DEFAULT 0 CHECK (allocation_cursor >= 0),
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE storage_pool_members (
    pool_uuid TEXT NOT NULL REFERENCES storage_pools(uuid) ON DELETE CASCADE,
    target_uuid TEXT NOT NULL REFERENCES storage_targets(uuid) ON DELETE RESTRICT,
    position INTEGER NOT NULL DEFAULT 0,
    weight INTEGER NOT NULL DEFAULT 1 CHECK (weight BETWEEN 1 AND 1000),
    PRIMARY KEY(pool_uuid, target_uuid)
);
CREATE INDEX idx_storage_pool_members_order
    ON storage_pool_members(pool_uuid, position, target_uuid);

ALTER TABLE storage_policies ADD COLUMN primary_pool_uuid TEXT
    REFERENCES storage_pools(uuid) ON DELETE RESTRICT;
ALTER TABLE storage_policies ADD COLUMN minimum_retention_days INTEGER NOT NULL DEFAULT 0
    CHECK (minimum_retention_days BETWEEN 0 AND 36500);
ALTER TABLE storage_policies ADD COLUMN desired_retention_days INTEGER NOT NULL DEFAULT 0
    CHECK (desired_retention_days BETWEEN 0 AND 36500);
ALTER TABLE storage_policies ADD COLUMN maximum_retention_days INTEGER NOT NULL DEFAULT 0
    CHECK (maximum_retention_days BETWEEN 0 AND 36500);
ALTER TABLE storage_policies ADD COLUMN required_copy_count INTEGER NOT NULL DEFAULT 1
    CHECK (required_copy_count BETWEEN 1 AND 8);
ALTER TABLE storage_policies ADD COLUMN replication_pool_uuid TEXT
    REFERENCES storage_pools(uuid) ON DELETE RESTRICT;
ALTER TABLE storage_policies ADD COLUMN migration_after_days INTEGER NOT NULL DEFAULT 0
    CHECK (migration_after_days BETWEEN 0 AND 36500);
ALTER TABLE storage_policies ADD COLUMN migration_target_uuid TEXT
    REFERENCES storage_targets(uuid) ON DELETE RESTRICT;
ALTER TABLE storage_policies ADD COLUMN pressure_priority INTEGER NOT NULL DEFAULT 100
    CHECK (pressure_priority BETWEEN -1000000 AND 1000000);

CREATE TABLE storage_policy_violations (
    uuid TEXT PRIMARY KEY,
    policy_uuid TEXT NOT NULL REFERENCES storage_policies(uuid) ON DELETE CASCADE,
    recording_id INTEGER REFERENCES recordings(id) ON DELETE CASCADE,
    camera_uuid TEXT,
    scope_key TEXT NOT NULL,
    violation_type TEXT NOT NULL
        CHECK (violation_type IN ('copy_count', 'minimum_retention',
                                  'target_unavailable', 'migration_failed')),
    details TEXT NOT NULL DEFAULT '',
    first_seen_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    last_seen_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    resolved_at INTEGER,
    UNIQUE(policy_uuid, scope_key, violation_type)
);
CREATE INDEX idx_storage_policy_violations_active
    ON storage_policy_violations(policy_uuid, resolved_at, violation_type);

-- migrate:down
DROP INDEX IF EXISTS idx_storage_policy_violations_active;
DROP TABLE IF EXISTS storage_policy_violations;
-- Pool tables and additive storage_policies columns are retained together for
-- safe rollback because SQLite cannot remove the referencing policy columns.
