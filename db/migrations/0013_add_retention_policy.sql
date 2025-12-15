-- Add retention policy support

-- migrate:up

-- Per-stream retention settings
ALTER TABLE streams ADD COLUMN retention_days INTEGER DEFAULT 30;
ALTER TABLE streams ADD COLUMN detection_retention_days INTEGER DEFAULT 90;
ALTER TABLE streams ADD COLUMN max_storage_mb INTEGER DEFAULT 0;

-- Recording protection
ALTER TABLE recordings ADD COLUMN protected INTEGER DEFAULT 0;
ALTER TABLE recordings ADD COLUMN retention_override_days INTEGER DEFAULT NULL;

-- Indexes for efficient retention queries
CREATE INDEX IF NOT EXISTS idx_recordings_protected ON recordings(protected);
CREATE INDEX IF NOT EXISTS idx_recordings_trigger_type ON recordings(trigger_type);

-- migrate:down

DROP INDEX IF EXISTS idx_recordings_trigger_type;
DROP INDEX IF EXISTS idx_recordings_protected;
SELECT 1;

