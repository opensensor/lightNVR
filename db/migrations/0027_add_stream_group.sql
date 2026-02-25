-- migrate:up
ALTER TABLE streams ADD COLUMN group_name TEXT NOT NULL DEFAULT '';

-- migrate:down
-- SQLite does not support DROP COLUMN in older versions; this is a no-op rollback
SELECT 1;

