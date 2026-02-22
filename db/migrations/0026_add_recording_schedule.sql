-- migrate:up
ALTER TABLE streams ADD COLUMN record_on_schedule INTEGER NOT NULL DEFAULT 0;
ALTER TABLE streams ADD COLUMN recording_schedule TEXT DEFAULT NULL;

-- migrate:down
-- SQLite does not support DROP COLUMN in older versions; this is a no-op rollback
SELECT 1;

