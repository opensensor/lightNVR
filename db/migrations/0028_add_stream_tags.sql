-- Add tags column to streams for comma-separated tag labels
-- Tags extend group_name to support multiple labels (e.g., "outdoor,critical,entrance")

-- migrate:up
ALTER TABLE streams ADD COLUMN tags TEXT NOT NULL DEFAULT '';

-- migrate:down
-- SQLite does not support DROP COLUMN in older versions; this is a no-op rollback
SELECT 1;

