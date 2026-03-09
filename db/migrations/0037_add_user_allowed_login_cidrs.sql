-- Add per-user login IP CIDR restrictions
-- NULL means unrestricted; non-NULL newline-separated CIDR list restricts auth to matching IPs

-- migrate:up
ALTER TABLE users ADD COLUMN allowed_login_cidrs TEXT DEFAULT NULL;

-- migrate:down
-- SQLite does not support DROP COLUMN in older versions; this is a no-op rollback
SELECT 1;