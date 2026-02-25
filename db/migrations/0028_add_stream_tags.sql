-- Introduce comma-separated tags and retire group_name in one migration.
--
-- Steps:
--   1. Add the new tags column.
--   2. Copy any existing group_name values into tags (where tags is still empty).
--   3. Drop the now-redundant group_name column.
--
-- Requires SQLite 3.35.0+ for ALTER TABLE … DROP COLUMN.

-- migrate:up
ALTER TABLE streams ADD COLUMN tags TEXT NOT NULL DEFAULT '';
UPDATE streams SET tags = group_name WHERE (tags = '' OR tags IS NULL) AND group_name IS NOT NULL AND group_name != '';
ALTER TABLE streams DROP COLUMN group_name;

-- migrate:down
-- Restore group_name from tags (best-effort: takes first tag only)
ALTER TABLE streams ADD COLUMN group_name TEXT NOT NULL DEFAULT '';
UPDATE streams SET group_name = CASE WHEN instr(tags, ',') > 0 THEN substr(tags, 1, instr(tags, ',') - 1) ELSE tags END WHERE tags != '';
ALTER TABLE streams DROP COLUMN tags;

