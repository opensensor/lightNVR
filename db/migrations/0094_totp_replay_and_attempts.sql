-- Persist account-wide OTP replay and failure state across challenges/restarts.
-- Existing enabled MFA secrets remain valid. Incomplete old enrollments must
-- start setup again because they have no bounded expiry.
-- migrate:up
ALTER TABLE users ADD COLUMN totp_last_used_step INTEGER NOT NULL DEFAULT -1;
ALTER TABLE users ADD COLUMN totp_failed_attempts INTEGER NOT NULL DEFAULT 0 CHECK (totp_failed_attempts >= 0);
ALTER TABLE users ADD COLUMN totp_window_started_at INTEGER NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN totp_setup_expires_at INTEGER NOT NULL DEFAULT 0;

-- migrate:down
-- Pending challenges must not survive removal of replay protection.
DELETE FROM sessions WHERE mfa_pending = 1;
ALTER TABLE users DROP COLUMN totp_setup_expires_at;
ALTER TABLE users DROP COLUMN totp_window_started_at;
ALTER TABLE users DROP COLUMN totp_failed_attempts;
ALTER TABLE users DROP COLUMN totp_last_used_step;
