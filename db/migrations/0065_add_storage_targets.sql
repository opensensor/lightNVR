-- Multi-target storage foundation.
--
-- Absolute file_path remains populated as a compatibility cache while the
-- application moves all recording consumers to target UUID + object key.

-- migrate:up

CREATE TABLE IF NOT EXISTS storage_targets (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL COLLATE NOCASE,
    target_type TEXT NOT NULL DEFAULT 'filesystem'
        CHECK (target_type IN ('filesystem')),
    root_path TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    is_default INTEGER NOT NULL DEFAULT 0 CHECK (is_default IN (0, 1)),
    storage_class TEXT NOT NULL DEFAULT 'hot'
        CHECK (storage_class IN ('hot', 'warm', 'cold')),
    reserve_bytes INTEGER NOT NULL DEFAULT 0 CHECK (reserve_bytes >= 0),
    high_watermark_pct REAL NOT NULL DEFAULT 90.0
        CHECK (high_watermark_pct > 0 AND high_watermark_pct < 100),
    low_watermark_pct REAL NOT NULL DEFAULT 80.0
        CHECK (low_watermark_pct >= 0 AND low_watermark_pct < high_watermark_pct),
    health_status TEXT NOT NULL DEFAULT 'unknown'
        CHECK (health_status IN ('unknown', 'healthy', 'degraded', 'unavailable', 'disabled')),
    capacity_bytes INTEGER NOT NULL DEFAULT 0,
    available_bytes INTEGER NOT NULL DEFAULT 0,
    filesystem_device INTEGER NOT NULL DEFAULT 0,
    last_probe_at INTEGER,
    last_success_at INTEGER,
    last_error TEXT NOT NULL DEFAULT '',
    recording_count INTEGER NOT NULL DEFAULT 0 CHECK (recording_count >= 0),
    recording_bytes INTEGER NOT NULL DEFAULT 0 CHECK (recording_bytes >= 0),
    revision INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_storage_targets_name
    ON storage_targets(name COLLATE NOCASE);
CREATE UNIQUE INDEX IF NOT EXISTS idx_storage_targets_root_path
    ON storage_targets(root_path);
CREATE UNIQUE INDEX IF NOT EXISTS idx_storage_targets_one_default
    ON storage_targets(is_default) WHERE is_default = 1;
CREATE INDEX IF NOT EXISTS idx_storage_targets_enabled
    ON storage_targets(enabled, name COLLATE NOCASE);

ALTER TABLE recordings ADD COLUMN storage_target_uuid TEXT
    REFERENCES storage_targets(uuid) ON DELETE SET NULL;
ALTER TABLE recordings ADD COLUMN object_key TEXT;
ALTER TABLE recordings ADD COLUMN placement_reason TEXT;
ALTER TABLE recordings ADD COLUMN storage_policy_version INTEGER;

CREATE INDEX IF NOT EXISTS idx_recordings_storage_target_time
    ON recordings(storage_target_uuid, start_time DESC);

CREATE TRIGGER IF NOT EXISTS trg_recordings_storage_target_insert
AFTER INSERT ON recordings
FOR EACH ROW WHEN NEW.storage_target_uuid IS NOT NULL
BEGIN
    UPDATE storage_targets
    SET recording_count = recording_count + 1,
        recording_bytes = recording_bytes + MAX(COALESCE(NEW.size_bytes, 0), 0)
    WHERE uuid = NEW.storage_target_uuid;
END;

CREATE TRIGGER IF NOT EXISTS trg_recordings_storage_target_delete
AFTER DELETE ON recordings
FOR EACH ROW WHEN OLD.storage_target_uuid IS NOT NULL
BEGIN
    UPDATE storage_targets
    SET recording_count = MAX(recording_count - 1, 0),
        recording_bytes = MAX(recording_bytes - MAX(COALESCE(OLD.size_bytes, 0), 0), 0)
    WHERE uuid = OLD.storage_target_uuid;
END;

CREATE TRIGGER IF NOT EXISTS trg_recordings_storage_target_update
AFTER UPDATE OF storage_target_uuid, size_bytes ON recordings
FOR EACH ROW
BEGIN
    UPDATE storage_targets
    SET recording_count = MAX(recording_count - 1, 0),
        recording_bytes = MAX(recording_bytes - MAX(COALESCE(OLD.size_bytes, 0), 0), 0)
    WHERE OLD.storage_target_uuid IS NOT NULL
      AND uuid = OLD.storage_target_uuid;
    UPDATE storage_targets
    SET recording_count = recording_count + 1,
        recording_bytes = recording_bytes + MAX(COALESCE(NEW.size_bytes, 0), 0)
    WHERE NEW.storage_target_uuid IS NOT NULL
      AND uuid = NEW.storage_target_uuid;
END;

-- migrate:down
DROP TRIGGER IF EXISTS trg_recordings_storage_target_update;
DROP TRIGGER IF EXISTS trg_recordings_storage_target_delete;
DROP TRIGGER IF EXISTS trg_recordings_storage_target_insert;
DROP INDEX IF EXISTS idx_recordings_storage_target_time;
DROP INDEX IF EXISTS idx_storage_targets_enabled;
DROP INDEX IF EXISTS idx_storage_targets_one_default;
DROP INDEX IF EXISTS idx_storage_targets_root_path;
DROP INDEX IF EXISTS idx_storage_targets_name;
DROP TABLE IF EXISTS storage_targets;
