-- Add password_change_locked column to users table for password lockdown support

-- migrate:up

-- Add password_change_locked column to users table
-- This allows admins to lock password changes for specific users (e.g., demo accounts)
ALTER TABLE users ADD COLUMN password_change_locked INTEGER DEFAULT 0;

-- migrate:down

-- SQLite doesn't support DROP COLUMN directly, so we need to recreate the table
-- For simplicity in the down migration, we'll just leave the column
-- (it's nullable with a default, so this won't break anything)


