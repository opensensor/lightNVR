-- Add TOTP MFA support to users table

-- migrate:up

ALTER TABLE users ADD COLUMN totp_secret TEXT;
ALTER TABLE users ADD COLUMN totp_enabled INTEGER DEFAULT 0;

-- migrate:down

-- SQLite doesn't support DROP COLUMN in older versions
-- The columns are nullable with safe defaults, so they can remain

