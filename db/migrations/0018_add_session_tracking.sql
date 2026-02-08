-- Add ip_address and user_agent columns to sessions table for tracking

-- migrate:up

-- Add ip_address column for tracking client IP
ALTER TABLE sessions ADD COLUMN ip_address TEXT;

-- Add user_agent column for tracking client user agent
ALTER TABLE sessions ADD COLUMN user_agent TEXT;

-- migrate:down

-- SQLite doesn't support DROP COLUMN directly, so we need to recreate the table
-- For simplicity in the down migration, we'll just leave the columns
-- (they're nullable, so this won't break anything)

