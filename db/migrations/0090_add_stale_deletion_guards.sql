-- Durable identity guards for narrow stale-incomplete recording cleanup.
-- Ordinary deletion objects retain NULL guards and their existing semantics.

-- migrate:up

ALTER TABLE storage_deletion_objects ADD COLUMN stale_guard_kind TEXT
    CHECK (stale_guard_kind IS NULL OR stale_guard_kind IN ('missing', 'empty'));
ALTER TABLE storage_deletion_objects ADD COLUMN stale_guard_dev INTEGER;
ALTER TABLE storage_deletion_objects ADD COLUMN stale_guard_ino INTEGER
    CHECK (CASE
        WHEN stale_guard_kind IS NULL THEN stale_guard_dev IS NULL AND stale_guard_ino IS NULL
        WHEN stale_guard_kind = 'missing' THEN stale_guard_dev IS NULL AND stale_guard_ino IS NULL
        WHEN stale_guard_kind = 'empty' THEN stale_guard_dev IS NOT NULL AND stale_guard_dev >= 0
            AND stale_guard_ino IS NOT NULL AND stale_guard_ino > 0
        ELSE 0
    END);

-- migrate:down

-- Removing these guards can make a pending operation delete replacement footage.
-- Downgrade is unsupported even when no guarded work currently appears pending;
-- do not run an older binary against this database. Restore a separately verified
-- pre-upgrade backup under a separately qualified recovery procedure instead.
CREATE TEMP TABLE IF NOT EXISTS stale_guard_downgrade_blocker (value INTEGER);
CREATE TEMP TRIGGER IF NOT EXISTS prevent_stale_guard_downgrade
BEFORE INSERT ON stale_guard_downgrade_blocker
BEGIN
    SELECT RAISE(ABORT, 'stale deletion guards cannot be downgraded');
END;
INSERT INTO stale_guard_downgrade_blocker VALUES (1);
