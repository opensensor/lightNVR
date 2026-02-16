-- migrate:up
ALTER TABLE streams ADD COLUMN detection_object_filter TEXT DEFAULT 'none';
ALTER TABLE streams ADD COLUMN detection_object_filter_list TEXT DEFAULT '';

-- migrate:down
-- SQLite doesn't support DROP COLUMN in older versions
-- These columns will just be ignored if not used

