-- Require a freshly bootstrapped default administrator to replace admin/admin.

-- migrate:up

ALTER TABLE users
ADD COLUMN must_change_password INTEGER NOT NULL DEFAULT 0
CHECK (must_change_password IN (0, 1));

-- migrate:down

-- Existing installations deliberately remain unflagged because the default is
-- zero and SQLite migration rollback does not need to rebuild the users table.
SELECT 1;
