-- Add allowed_tags column to users for tag-based RBAC
-- NULL means no restriction (user can see all streams)
-- Non-NULL comma-separated list restricts access to streams matching at least one tag

-- migrate:up
ALTER TABLE users ADD COLUMN allowed_tags TEXT DEFAULT NULL;

-- migrate:down
-- SQLite does not support DROP COLUMN in older versions; this is a no-op rollback
SELECT 1;

